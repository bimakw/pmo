'use client';

import { useState, useEffect, useCallback } from 'react';
import { Header } from '@/components/layout/header';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { tasksApi, projectsApi, timeLogsApi } from '@/lib/api';
import type { Task, Project, TimeLog } from '@/types';
import {
  Clock,
  Play,
  Square,
  Plus,
  ChevronLeft,
  ChevronRight,
  Calendar,
  Trash2,
} from 'lucide-react';

function getWeekDates(date: Date): Date[] {
  const week: Date[] = [];
  const start = new Date(date);
  start.setDate(start.getDate() - start.getDay() + 1); // Monday

  for (let i = 0; i < 7; i++) {
    const day = new Date(start);
    day.setDate(start.getDate() + i);
    week.push(day);
  }
  return week;
}

function formatDate(date: Date): string {
  return date.toISOString().split('T')[0];
}

function formatDisplayDate(date: Date): string {
  return date.toLocaleDateString('id-ID', { weekday: 'short', day: 'numeric' });
}

interface TimeEntryModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (entry: { task_id: string; hours: number; date: string; description: string }) => void;
  tasks: Task[];
  selectedDate: string;
  isSubmitting: boolean;
}

function TimeEntryModal({ isOpen, onClose, onSubmit, tasks, selectedDate, isSubmitting }: TimeEntryModalProps) {
  const [taskId, setTaskId] = useState('');
  const [hours, setHours] = useState('');
  const [description, setDescription] = useState('');

  if (!isOpen) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!taskId || !hours) return;
    onSubmit({
      task_id: taskId,
      hours: parseFloat(hours),
      date: selectedDate,
      description,
    });
  };

  const handleClose = () => {
    setTaskId('');
    setHours('');
    setDescription('');
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 w-full max-w-md">
        <h3 className="text-lg font-semibold mb-4">Log Time</h3>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Task</label>
            <select
              value={taskId}
              onChange={(e) => setTaskId(e.target.value)}
              className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
              disabled={isSubmitting}
            >
              <option value="">Select task...</option>
              {tasks.map((task) => (
                <option key={task.id} value={task.id}>
                  {task.title}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Hours</label>
            <Input
              type="number"
              step="0.25"
              min="0.25"
              max="24"
              value={hours}
              onChange={(e) => setHours(e.target.value)}
              placeholder="e.g. 2.5"
              required
              disabled={isSubmitting}
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Description (optional)</label>
            <Input
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="What did you work on?"
              disabled={isSubmitting}
            />
          </div>
          <div className="flex gap-2 justify-end">
            <Button type="button" variant="outline" onClick={handleClose} disabled={isSubmitting}>
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? 'Saving...' : 'Save'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default function TimesheetPage() {
  const [timeLogs, setTimeLogs] = useState<TimeLog[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [currentWeek, setCurrentWeek] = useState(new Date());
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedDate, setSelectedDate] = useState(formatDate(new Date()));
  const [activeTimer, setActiveTimer] = useState<{ taskId: string; startTime: Date } | null>(null);
  const [timerElapsed, setTimerElapsed] = useState(0);

  const weekDates = getWeekDates(currentWeek);

  const loadTimeLogs = useCallback(async () => {
    try {
      const startDate = formatDate(weekDates[0]);
      const endDate = formatDate(weekDates[6]);
      const response = await timeLogsApi.list({ start_date: startDate, end_date: endDate });
      if (response.data) {
        setTimeLogs(response.data);
      }
    } catch (error) {
      console.error('Failed to load time logs:', error);
    }
  }, [weekDates]);

  useEffect(() => {
    const loadData = async () => {
      try {
        const [tasksRes, projectsRes] = await Promise.all([
          tasksApi.list(),
          projectsApi.list(),
        ]);

        if (tasksRes.data) setTasks(tasksRes.data);
        if (projectsRes.data) setProjects(projectsRes.data);
      } catch (error) {
        console.error('Failed to load data:', error);
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, []);

  useEffect(() => {
    if (!loading) {
      loadTimeLogs();
    }
  }, [loading, currentWeek, loadTimeLogs]);

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (activeTimer) {
      interval = setInterval(() => {
        setTimerElapsed(Math.floor((Date.now() - activeTimer.startTime.getTime()) / 1000));
      }, 1000);
    }
    return () => clearInterval(interval);
  }, [activeTimer]);

  const handlePrevWeek = () => {
    const newDate = new Date(currentWeek);
    newDate.setDate(newDate.getDate() - 7);
    setCurrentWeek(newDate);
  };

  const handleNextWeek = () => {
    const newDate = new Date(currentWeek);
    newDate.setDate(newDate.getDate() + 7);
    setCurrentWeek(newDate);
  };

  const handleAddEntry = async (entry: { task_id: string; hours: number; date: string; description: string }) => {
    setIsSubmitting(true);
    try {
      const response = await timeLogsApi.create({
        task_id: entry.task_id,
        hours: entry.hours,
        date: entry.date,
        description: entry.description || undefined,
      });

      if (response.data) {
        setTimeLogs([response.data, ...timeLogs]);
        setIsModalOpen(false);
      }
    } catch (error) {
      console.error('Failed to create time log:', error);
      alert('Failed to save time entry');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDeleteEntry = async (id: string) => {
    if (!confirm('Are you sure you want to delete this entry?')) return;

    try {
      await timeLogsApi.delete(id);
      setTimeLogs(timeLogs.filter((log) => log.id !== id));
    } catch (error) {
      console.error('Failed to delete time log:', error);
      alert('Failed to delete time entry');
    }
  };

  const handleStartTimer = (taskId: string) => {
    setActiveTimer({ taskId, startTime: new Date() });
    setTimerElapsed(0);
  };

  const handleStopTimer = async () => {
    if (!activeTimer) return;

    const hours = Math.round((timerElapsed / 3600) * 4) / 4; // Round to nearest 0.25
    if (hours >= 0.25) {
      await handleAddEntry({
        task_id: activeTimer.taskId,
        hours,
        date: formatDate(new Date()),
        description: 'Timer entry',
      });
    }

    setActiveTimer(null);
    setTimerElapsed(0);
  };

  const formatTimer = (seconds: number): string => {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  };

  const getHoursForDate = (date: string): number => {
    return timeLogs
      .filter((log) => log.date === date)
      .reduce((sum, log) => sum + log.hours, 0);
  };

  const getWeekTotal = (): number => {
    return weekDates.reduce((sum, date) => sum + getHoursForDate(formatDate(date)), 0);
  };

  const todayLogs = timeLogs.filter((log) => log.date === formatDate(new Date()));

  if (loading) {
    return (
      <div>
        <Header title="Timesheet" />
        <div className="flex justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
        </div>
      </div>
    );
  }

  return (
    <div>
      <Header title="Timesheet" />

      <div className="p-6">
        {/* Timer Section */}
        {tasks.length > 0 && (
          <Card className="mb-6">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Clock className="h-5 w-5" />
                Quick Timer
              </CardTitle>
            </CardHeader>
            <CardContent>
              {activeTimer ? (
                <div className="flex items-center gap-4">
                  <div className="text-3xl font-mono font-bold text-blue-600">
                    {formatTimer(timerElapsed)}
                  </div>
                  <div className="flex-1">
                    <p className="text-sm text-gray-600">
                      Working on: {tasks.find((t) => t.id === activeTimer.taskId)?.title}
                    </p>
                  </div>
                  <Button onClick={handleStopTimer} variant="danger">
                    <Square className="h-4 w-4 mr-2" />
                    Stop
                  </Button>
                </div>
              ) : (
                <div className="flex items-center gap-4">
                  <select
                    className="flex-1 px-3 py-2 border rounded-lg"
                    id="timer-task"
                    defaultValue=""
                  >
                    <option value="">Select task to start timer...</option>
                    {tasks.filter((t) => t.status !== 'Done').map((task) => (
                      <option key={task.id} value={task.id}>
                        {task.title}
                      </option>
                    ))}
                  </select>
                  <Button
                    onClick={() => {
                      const select = document.getElementById('timer-task') as HTMLSelectElement;
                      if (select.value) handleStartTimer(select.value);
                    }}
                  >
                    <Play className="h-4 w-4 mr-2" />
                    Start Timer
                  </Button>
                </div>
              )}
            </CardContent>
          </Card>
        )}

        {/* Weekly View */}
        <Card className="mb-6">
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle className="flex items-center gap-2">
                <Calendar className="h-5 w-5" />
                Weekly Timesheet
              </CardTitle>
              <div className="flex items-center gap-2">
                <Button variant="outline" size="sm" onClick={handlePrevWeek}>
                  <ChevronLeft className="h-4 w-4" />
                </Button>
                <span className="text-sm font-medium px-2">
                  {weekDates[0].toLocaleDateString('id-ID', { month: 'short', day: 'numeric' })} -{' '}
                  {weekDates[6].toLocaleDateString('id-ID', { month: 'short', day: 'numeric', year: 'numeric' })}
                </span>
                <Button variant="outline" size="sm" onClick={handleNextWeek}>
                  <ChevronRight className="h-4 w-4" />
                </Button>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-8 gap-2">
              {/* Header */}
              {weekDates.map((date) => (
                <div
                  key={formatDate(date)}
                  className={`text-center p-2 rounded-lg ${
                    formatDate(date) === formatDate(new Date())
                      ? 'bg-blue-100 text-blue-800'
                      : 'bg-gray-50'
                  }`}
                >
                  <div className="text-xs font-medium">{formatDisplayDate(date)}</div>
                  <div className="text-lg font-bold mt-1">{getHoursForDate(formatDate(date))}h</div>
                </div>
              ))}
              <div className="text-center p-2 rounded-lg bg-green-100">
                <div className="text-xs font-medium text-green-800">Total</div>
                <div className="text-lg font-bold text-green-800 mt-1">{getWeekTotal()}h</div>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Today's Entries */}
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle>Today&apos;s Time Entries</CardTitle>
              <Button
                size="sm"
                onClick={() => {
                  setSelectedDate(formatDate(new Date()));
                  setIsModalOpen(true);
                }}
              >
                <Plus className="h-4 w-4 mr-2" />
                Log Time
              </Button>
            </div>
          </CardHeader>
          <CardContent>
            {todayLogs.length === 0 ? (
              <div className="text-center py-8 text-gray-500">
                <Clock className="h-12 w-12 mx-auto mb-4 text-gray-300" />
                <p>No time logged today</p>
                <p className="text-sm">Start a timer or add an entry manually</p>
              </div>
            ) : (
              <div className="space-y-3">
                {todayLogs.map((log) => (
                  <div
                    key={log.id}
                    className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
                  >
                    <div className="flex-1">
                      <p className="font-medium text-sm">{log.task_name}</p>
                      {log.project_name && (
                        <Badge variant="secondary" className="text-xs mt-1">
                          {log.project_name}
                        </Badge>
                      )}
                      {log.description && (
                        <p className="text-xs text-gray-500 mt-1">{log.description}</p>
                      )}
                    </div>
                    <div className="flex items-center gap-3">
                      <span className="text-lg font-bold text-blue-600">{log.hours}h</span>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleDeleteEntry(log.id)}
                        className="text-red-500 hover:text-red-700"
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  </div>
                ))}
                <div className="flex justify-end pt-2 border-t">
                  <span className="font-medium">
                    Total: <span className="text-blue-600">{todayLogs.reduce((sum, log) => sum + log.hours, 0)}h</span>
                  </span>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      <TimeEntryModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onSubmit={handleAddEntry}
        tasks={tasks}
        selectedDate={selectedDate}
        isSubmitting={isSubmitting}
      />
    </div>
  );
}
