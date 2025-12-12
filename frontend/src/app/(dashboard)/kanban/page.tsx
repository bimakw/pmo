'use client';

import { useState, useEffect, useCallback } from 'react';
import { Header } from '@/components/layout/header';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { tasksApi, projectsApi } from '@/lib/api';
import type { Task, Project, TaskStatus, Priority } from '@/types';
import {
  GripVertical,
  Clock,
  User,
  Filter,
  ChevronDown,
} from 'lucide-react';

const COLUMNS: { id: TaskStatus; title: string; color: string }[] = [
  { id: 'Todo', title: 'To Do', color: 'bg-gray-500' },
  { id: 'inprogress', title: 'In Progress', color: 'bg-blue-500' },
  { id: 'Review', title: 'Review', color: 'bg-purple-500' },
  { id: 'Done', title: 'Done', color: 'bg-green-500' },
  { id: 'Blocked', title: 'Blocked', color: 'bg-red-500' },
];

const priorityColors: Record<Priority, string> = {
  Low: 'bg-gray-100 text-gray-600',
  Medium: 'bg-blue-100 text-blue-600',
  High: 'bg-orange-100 text-orange-600',
  Critical: 'bg-red-100 text-red-600',
};

interface KanbanCardProps {
  task: Task;
  onDragStart: (e: React.DragEvent, task: Task) => void;
}

function KanbanCard({ task, onDragStart }: KanbanCardProps) {
  const formatDate = (dateString?: string) => {
    if (!dateString) return null;
    const date = new Date(dateString);
    const isOverdue = date < new Date() && task.status !== 'Done';
    return {
      formatted: date.toLocaleDateString('id-ID', { day: 'numeric', month: 'short' }),
      isOverdue,
    };
  };

  const dueDate = formatDate(task.due_date);

  return (
    <div
      draggable
      onDragStart={(e) => onDragStart(e, task)}
      className="bg-white rounded-lg shadow-sm border border-gray-200 p-3 cursor-grab active:cursor-grabbing hover:shadow-md transition-shadow group"
    >
      <div className="flex items-start gap-2">
        <GripVertical className="h-4 w-4 text-gray-400 mt-1 opacity-0 group-hover:opacity-100 transition-opacity" />
        <div className="flex-1 min-w-0">
          <h4 className="font-medium text-sm text-gray-900 truncate">{task.title}</h4>
          {task.description && (
            <p className="text-xs text-gray-500 mt-1 line-clamp-2">{task.description}</p>
          )}
          <div className="flex items-center gap-2 mt-2 flex-wrap">
            <Badge className={priorityColors[task.priority]}>{task.priority}</Badge>
            {dueDate && (
              <span
                className={`text-xs flex items-center gap-1 ${
                  dueDate.isOverdue ? 'text-red-600' : 'text-gray-500'
                }`}
              >
                <Clock className="h-3 w-3" />
                {dueDate.formatted}
              </span>
            )}
            {task.assignee_id && (
              <span className="text-xs flex items-center gap-1 text-gray-500">
                <User className="h-3 w-3" />
                Assigned
              </span>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

interface KanbanColumnProps {
  column: (typeof COLUMNS)[0];
  tasks: Task[];
  onDragStart: (e: React.DragEvent, task: Task) => void;
  onDragOver: (e: React.DragEvent) => void;
  onDrop: (e: React.DragEvent, status: TaskStatus) => void;
  isDragOver: boolean;
}

function KanbanColumn({
  column,
  tasks,
  onDragStart,
  onDragOver,
  onDrop,
  isDragOver,
}: KanbanColumnProps) {
  return (
    <div className="flex-shrink-0 w-72">
      <div className="bg-gray-100 rounded-lg p-3">
        <div className="flex items-center gap-2 mb-3">
          <div className={`w-3 h-3 rounded-full ${column.color}`} />
          <h3 className="font-semibold text-sm text-gray-700">{column.title}</h3>
          <Badge variant="secondary" className="ml-auto">
            {tasks.length}
          </Badge>
        </div>
        <div
          onDragOver={onDragOver}
          onDrop={(e) => onDrop(e, column.id)}
          className={`space-y-2 min-h-[200px] transition-colors rounded-lg p-1 ${
            isDragOver ? 'bg-blue-50 border-2 border-dashed border-blue-300' : ''
          }`}
        >
          {tasks.map((task) => (
            <KanbanCard key={task.id} task={task} onDragStart={onDragStart} />
          ))}
          {tasks.length === 0 && (
            <div className="text-center py-8 text-sm text-gray-400">
              No tasks
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default function KanbanPage() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedProject, setSelectedProject] = useState<string>('all');
  const [draggedTask, setDraggedTask] = useState<Task | null>(null);
  const [dragOverColumn, setDragOverColumn] = useState<TaskStatus | null>(null);
  const [showProjectDropdown, setShowProjectDropdown] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

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

  const handleDragStart = useCallback((e: React.DragEvent, task: Task) => {
    setDraggedTask(task);
    e.dataTransfer.effectAllowed = 'move';
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
  }, []);

  const handleDrop = useCallback(
    async (e: React.DragEvent, newStatus: TaskStatus) => {
      e.preventDefault();
      setDragOverColumn(null);

      if (!draggedTask || draggedTask.status === newStatus) {
        setDraggedTask(null);
        return;
      }

      // Optimistic update
      setTasks((prev) =>
        prev.map((t) =>
          t.id === draggedTask.id ? { ...t, status: newStatus } : t
        )
      );

      try {
        await tasksApi.update(draggedTask.id, { status: newStatus });
      } catch (error) {
        console.error('Failed to update task status:', error);
        // Revert on error
        setTasks((prev) =>
          prev.map((t) =>
            t.id === draggedTask.id ? { ...t, status: draggedTask.status } : t
          )
        );
      }

      setDraggedTask(null);
    },
    [draggedTask]
  );

  const handleColumnDragOver = useCallback((status: TaskStatus) => {
    setDragOverColumn(status);
  }, []);

  const filteredTasks =
    selectedProject === 'all'
      ? tasks
      : tasks.filter((t) => t.project_id === selectedProject);

  const getTasksByStatus = (status: TaskStatus) =>
    filteredTasks.filter((t) => t.status === status);

  if (loading) {
    return (
      <div>
        <Header title="Kanban Board" />
        <div className="flex justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
        </div>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      <Header title="Kanban Board" />

      <div className="p-4 border-b bg-white">
        <div className="flex items-center gap-4">
          <div className="relative">
            <button
              onClick={() => setShowProjectDropdown(!showProjectDropdown)}
              className="flex items-center gap-2 px-4 py-2 border rounded-lg hover:bg-gray-50"
            >
              <Filter className="h-4 w-4" />
              <span className="text-sm">
                {selectedProject === 'all'
                  ? 'All Projects'
                  : projects.find((p) => p.id === selectedProject)?.name || 'Select'}
              </span>
              <ChevronDown className="h-4 w-4" />
            </button>

            {showProjectDropdown && (
              <div className="absolute top-full left-0 mt-1 w-64 bg-white border rounded-lg shadow-lg z-10">
                <button
                  onClick={() => {
                    setSelectedProject('all');
                    setShowProjectDropdown(false);
                  }}
                  className={`w-full text-left px-4 py-2 hover:bg-gray-50 text-sm ${
                    selectedProject === 'all' ? 'bg-blue-50 text-blue-600' : ''
                  }`}
                >
                  All Projects
                </button>
                {projects.map((project) => (
                  <button
                    key={project.id}
                    onClick={() => {
                      setSelectedProject(project.id);
                      setShowProjectDropdown(false);
                    }}
                    className={`w-full text-left px-4 py-2 hover:bg-gray-50 text-sm ${
                      selectedProject === project.id ? 'bg-blue-50 text-blue-600' : ''
                    }`}
                  >
                    {project.name}
                  </button>
                ))}
              </div>
            )}
          </div>

          <div className="text-sm text-gray-500">
            {filteredTasks.length} tasks
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-x-auto p-4">
        <div className="flex gap-4 min-w-max">
          {COLUMNS.map((column) => (
            <KanbanColumn
              key={column.id}
              column={column}
              tasks={getTasksByStatus(column.id)}
              onDragStart={handleDragStart}
              onDragOver={(e) => {
                handleDragOver(e);
                handleColumnDragOver(column.id);
              }}
              onDrop={handleDrop}
              isDragOver={dragOverColumn === column.id}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
