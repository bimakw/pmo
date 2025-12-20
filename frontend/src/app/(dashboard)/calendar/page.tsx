'use client';

import { useState, useEffect } from 'react';
import { format } from 'date-fns';
import { X, CheckSquare, Flag, Clock, User } from 'lucide-react';
import { Header } from '@/components/layout/header';
import { Calendar } from '@/components/ui/calendar';
import { Badge } from '@/components/ui/badge';
import { tasksApi, projectsApi } from '@/lib/api';
import type { Task, Milestone, Priority, TaskStatus } from '@/types';

const priorityColors: Record<Priority, string> = {
  Low: 'bg-gray-100 text-gray-600',
  Medium: 'bg-blue-100 text-blue-600',
  High: 'bg-orange-100 text-orange-600',
  Critical: 'bg-red-100 text-red-600',
};

const statusColors: Record<TaskStatus, string> = {
  Todo: 'bg-gray-100 text-gray-800',
  inprogress: 'bg-blue-100 text-blue-800',
  Review: 'bg-purple-100 text-purple-800',
  Done: 'bg-green-100 text-green-800',
  Blocked: 'bg-red-100 text-red-800',
};

const statusLabels: Record<TaskStatus, string> = {
  Todo: 'To Do',
  inprogress: 'In Progress',
  Review: 'Review',
  Done: 'Done',
  Blocked: 'Blocked',
};

interface CalendarEvent {
  id: string;
  title: string;
  date: string;
  type: 'task' | 'milestone';
  status?: string;
  priority?: string;
}

export default function CalendarPage() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [milestones, setMilestones] = useState<Milestone[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedEvent, setSelectedEvent] = useState<CalendarEvent | null>(null);
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);
  const [selectedMilestone, setSelectedMilestone] = useState<Milestone | null>(null);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [tasksResponse, projectsResponse] = await Promise.all([
        tasksApi.list(),
        projectsApi.list(),
      ]);

      if (tasksResponse.data) {
        setTasks(tasksResponse.data);
      }

      if (projectsResponse.data) {
        const allMilestones: Milestone[] = [];
        for (const project of projectsResponse.data) {
          const milestonesResponse = await projectsApi.getMilestones(project.id);
          if (milestonesResponse.data) {
            allMilestones.push(...milestonesResponse.data);
          }
        }
        setMilestones(allMilestones);
      }
    } catch (error) {
      console.error('Failed to load data:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDateClick = (date: Date) => {
    setSelectedDate(date);
    setSelectedEvent(null);
    setSelectedTask(null);
    setSelectedMilestone(null);
  };

  const handleEventClick = (event: CalendarEvent) => {
    setSelectedEvent(event);
    setSelectedDate(null);

    if (event.type === 'task') {
      const task = tasks.find((t) => t.id === event.id);
      setSelectedTask(task || null);
      setSelectedMilestone(null);
    } else {
      const milestone = milestones.find((m) => m.id === event.id);
      setSelectedMilestone(milestone || null);
      setSelectedTask(null);
    }
  };

  const closePanel = () => {
    setSelectedEvent(null);
    setSelectedDate(null);
    setSelectedTask(null);
    setSelectedMilestone(null);
  };

  const getTasksForDate = (date: Date) => {
    return tasks.filter(
      (task) =>
        task.due_date &&
        format(new Date(task.due_date), 'yyyy-MM-dd') === format(date, 'yyyy-MM-dd')
    );
  };

  const getMilestonesForDate = (date: Date) => {
    return milestones.filter(
      (m) =>
        m.due_date &&
        format(new Date(m.due_date), 'yyyy-MM-dd') === format(date, 'yyyy-MM-dd')
    );
  };

  const upcomingTasks = tasks
    .filter((task) => task.due_date && new Date(task.due_date) >= new Date())
    .sort((a, b) => new Date(a.due_date!).getTime() - new Date(b.due_date!).getTime())
    .slice(0, 5);

  const overdueTasks = tasks
    .filter(
      (task) =>
        task.due_date &&
        new Date(task.due_date) < new Date() &&
        task.status !== 'Done'
    )
    .sort((a, b) => new Date(b.due_date!).getTime() - new Date(a.due_date!).getTime());

  return (
    <div>
      <Header title="Calendar" />

      <div className="p-6">
        {loading ? (
          <div className="flex justify-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
          </div>
        ) : (
          <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
            {/* Calendar */}
            <div className="lg:col-span-3">
              <Calendar
                tasks={tasks}
                milestones={milestones}
                onDateClick={handleDateClick}
                onEventClick={handleEventClick}
              />
            </div>

            {/* Sidebar */}
            <div className="space-y-6">
              {/* Selected Event/Date Panel */}
              {(selectedEvent || selectedDate) && (
                <div className="bg-white rounded-lg shadow p-4">
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="font-semibold text-gray-900">
                      {selectedDate
                        ? format(selectedDate, 'MMMM d, yyyy')
                        : selectedEvent?.type === 'task'
                        ? 'Task Details'
                        : 'Milestone Details'}
                    </h3>
                    <button
                      onClick={closePanel}
                      className="p-1 hover:bg-gray-100 rounded"
                    >
                      <X className="h-4 w-4 text-gray-500" />
                    </button>
                  </div>

                  {/* Date View */}
                  {selectedDate && (
                    <div className="space-y-3">
                      {getTasksForDate(selectedDate).length === 0 &&
                      getMilestonesForDate(selectedDate).length === 0 ? (
                        <p className="text-sm text-gray-500">
                          No events on this date
                        </p>
                      ) : (
                        <>
                          {getTasksForDate(selectedDate).map((task) => (
                            <div
                              key={task.id}
                              className="p-3 bg-gray-50 rounded-lg cursor-pointer hover:bg-gray-100"
                              onClick={() =>
                                handleEventClick({
                                  id: task.id,
                                  title: task.title,
                                  date: task.due_date!,
                                  type: 'task',
                                  status: task.status,
                                  priority: task.priority,
                                })
                              }
                            >
                              <div className="flex items-center gap-2">
                                <CheckSquare className="h-4 w-4 text-blue-600" />
                                <span className="text-sm font-medium">
                                  {task.title}
                                </span>
                              </div>
                              <div className="flex gap-2 mt-2">
                                <Badge className={statusColors[task.status]}>
                                  {statusLabels[task.status]}
                                </Badge>
                                <Badge className={priorityColors[task.priority]}>
                                  {task.priority}
                                </Badge>
                              </div>
                            </div>
                          ))}
                          {getMilestonesForDate(selectedDate).map((milestone) => (
                            <div
                              key={milestone.id}
                              className="p-3 bg-purple-50 rounded-lg cursor-pointer hover:bg-purple-100"
                              onClick={() =>
                                handleEventClick({
                                  id: milestone.id,
                                  title: milestone.name,
                                  date: milestone.due_date!,
                                  type: 'milestone',
                                })
                              }
                            >
                              <div className="flex items-center gap-2">
                                <Flag className="h-4 w-4 text-purple-600" />
                                <span className="text-sm font-medium">
                                  {milestone.name}
                                </span>
                              </div>
                            </div>
                          ))}
                        </>
                      )}
                    </div>
                  )}

                  {/* Task Detail View */}
                  {selectedTask && (
                    <div className="space-y-3">
                      <h4 className="font-medium text-gray-900">
                        {selectedTask.title}
                      </h4>
                      {selectedTask.description && (
                        <p className="text-sm text-gray-600">
                          {selectedTask.description}
                        </p>
                      )}
                      <div className="flex flex-wrap gap-2">
                        <Badge className={statusColors[selectedTask.status]}>
                          {statusLabels[selectedTask.status]}
                        </Badge>
                        <Badge className={priorityColors[selectedTask.priority]}>
                          {selectedTask.priority}
                        </Badge>
                      </div>
                      {selectedTask.due_date && (
                        <div className="flex items-center gap-2 text-sm text-gray-600">
                          <Clock className="h-4 w-4" />
                          Due: {format(new Date(selectedTask.due_date), 'MMM d, yyyy')}
                        </div>
                      )}
                      {selectedTask.estimated_hours && (
                        <div className="text-sm text-gray-600">
                          Estimated: {selectedTask.estimated_hours}h
                        </div>
                      )}
                    </div>
                  )}

                  {/* Milestone Detail View */}
                  {selectedMilestone && (
                    <div className="space-y-3">
                      <h4 className="font-medium text-gray-900">
                        {selectedMilestone.name}
                      </h4>
                      {selectedMilestone.description && (
                        <p className="text-sm text-gray-600">
                          {selectedMilestone.description}
                        </p>
                      )}
                      <Badge
                        className={
                          selectedMilestone.completed
                            ? 'bg-green-100 text-green-800'
                            : 'bg-yellow-100 text-yellow-800'
                        }
                      >
                        {selectedMilestone.completed ? 'Completed' : 'In Progress'}
                      </Badge>
                      {selectedMilestone.due_date && (
                        <div className="flex items-center gap-2 text-sm text-gray-600">
                          <Clock className="h-4 w-4" />
                          Due: {format(new Date(selectedMilestone.due_date), 'MMM d, yyyy')}
                        </div>
                      )}
                    </div>
                  )}
                </div>
              )}

              {/* Overdue Tasks */}
              {overdueTasks.length > 0 && (
                <div className="bg-white rounded-lg shadow p-4">
                  <h3 className="font-semibold text-red-600 mb-3">
                    Overdue Tasks ({overdueTasks.length})
                  </h3>
                  <div className="space-y-2">
                    {overdueTasks.slice(0, 5).map((task) => (
                      <div
                        key={task.id}
                        className="p-2 bg-red-50 rounded text-sm cursor-pointer hover:bg-red-100"
                        onClick={() =>
                          handleEventClick({
                            id: task.id,
                            title: task.title,
                            date: task.due_date!,
                            type: 'task',
                            status: task.status,
                            priority: task.priority,
                          })
                        }
                      >
                        <div className="font-medium text-gray-900 truncate">
                          {task.title}
                        </div>
                        <div className="text-xs text-red-600">
                          Due: {format(new Date(task.due_date!), 'MMM d')}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Upcoming Tasks */}
              <div className="bg-white rounded-lg shadow p-4">
                <h3 className="font-semibold text-gray-900 mb-3">Upcoming Tasks</h3>
                {upcomingTasks.length === 0 ? (
                  <p className="text-sm text-gray-500">No upcoming tasks</p>
                ) : (
                  <div className="space-y-2">
                    {upcomingTasks.map((task) => (
                      <div
                        key={task.id}
                        className="p-2 bg-gray-50 rounded text-sm cursor-pointer hover:bg-gray-100"
                        onClick={() =>
                          handleEventClick({
                            id: task.id,
                            title: task.title,
                            date: task.due_date!,
                            type: 'task',
                            status: task.status,
                            priority: task.priority,
                          })
                        }
                      >
                        <div className="font-medium text-gray-900 truncate">
                          {task.title}
                        </div>
                        <div className="text-xs text-gray-500">
                          Due: {format(new Date(task.due_date!), 'MMM d')}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
