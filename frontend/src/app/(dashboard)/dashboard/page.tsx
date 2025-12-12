'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import {
  FolderKanban,
  CheckSquare,
  Users,
  TrendingUp,
  Clock,
  AlertTriangle,
  ArrowRight,
} from 'lucide-react';
import {
  PieChart,
  Pie,
  Cell,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from 'recharts';
import { Header } from '@/components/layout/header';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { projectsApi, tasksApi, teamsApi } from '@/lib/api';
import type { Project, Task, Team, ProjectStatus, TaskStatus, Priority } from '@/types';

const statusColors: Record<ProjectStatus, string> = {
  Planning: 'bg-gray-100 text-gray-800',
  Active: 'bg-green-100 text-green-800',
  OnHold: 'bg-yellow-100 text-yellow-800',
  Completed: 'bg-blue-100 text-blue-800',
  Cancelled: 'bg-red-100 text-red-800',
};

const taskStatusColors: Record<TaskStatus, string> = {
  Todo: 'bg-gray-100 text-gray-800',
  inprogress: 'bg-blue-100 text-blue-800',
  Review: 'bg-purple-100 text-purple-800',
  Done: 'bg-green-100 text-green-800',
  Blocked: 'bg-red-100 text-red-800',
};

const priorityColors: Record<Priority, string> = {
  Low: 'bg-gray-100 text-gray-600',
  Medium: 'bg-blue-100 text-blue-600',
  High: 'bg-orange-100 text-orange-600',
  Critical: 'bg-red-100 text-red-600',
};

export default function DashboardPage() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [teams, setTeams] = useState<Team[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [projectsRes, tasksRes, teamsRes] = await Promise.all([
        projectsApi.list(),
        tasksApi.list(),
        teamsApi.list(),
      ]);

      if (projectsRes.data) setProjects(projectsRes.data);
      if (tasksRes.data) setTasks(tasksRes.data);
      if (teamsRes.data) setTeams(teamsRes.data);
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  const stats = {
    totalProjects: projects.length,
    activeProjects: projects.filter((p) => p.status === 'Active').length,
    totalTasks: tasks.length,
    completedTasks: tasks.filter((t) => t.status === 'Done').length,
    inProgressTasks: tasks.filter((t) => t.status === 'inprogress').length,
    overdueTasks: tasks.filter(
      (t) => t.due_date && new Date(t.due_date) < new Date() && t.status !== 'Done'
    ).length,
    totalTeams: teams.length,
  };

  // Chart data preparation
  const projectStatusData = [
    { name: 'Planning', value: projects.filter((p) => p.status === 'Planning').length, color: '#9CA3AF' },
    { name: 'Active', value: projects.filter((p) => p.status === 'Active').length, color: '#10B981' },
    { name: 'On Hold', value: projects.filter((p) => p.status === 'OnHold').length, color: '#F59E0B' },
    { name: 'Completed', value: projects.filter((p) => p.status === 'Completed').length, color: '#3B82F6' },
    { name: 'Cancelled', value: projects.filter((p) => p.status === 'Cancelled').length, color: '#EF4444' },
  ].filter((item) => item.value > 0);

  const taskStatusData = [
    { name: 'To Do', value: tasks.filter((t) => t.status === 'Todo').length, color: '#9CA3AF' },
    { name: 'In Progress', value: tasks.filter((t) => t.status === 'inprogress').length, color: '#3B82F6' },
    { name: 'Review', value: tasks.filter((t) => t.status === 'Review').length, color: '#8B5CF6' },
    { name: 'Done', value: tasks.filter((t) => t.status === 'Done').length, color: '#10B981' },
    { name: 'Blocked', value: tasks.filter((t) => t.status === 'Blocked').length, color: '#EF4444' },
  ].filter((item) => item.value > 0);

  const taskPriorityData = [
    { name: 'Low', value: tasks.filter((t) => t.priority === 'Low').length },
    { name: 'Medium', value: tasks.filter((t) => t.priority === 'Medium').length },
    { name: 'High', value: tasks.filter((t) => t.priority === 'High').length },
    { name: 'Critical', value: tasks.filter((t) => t.priority === 'Critical').length },
  ];

  const recentProjects = projects.slice(0, 5);
  const urgentTasks = tasks
    .filter((t) => t.status !== 'Done' && (t.priority === 'High' || t.priority === 'Critical'))
    .slice(0, 5);

  const formatDate = (dateString?: string) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleDateString('id-ID', {
      day: 'numeric',
      month: 'short',
    });
  };

  if (loading) {
    return (
      <div>
        <Header title="Dashboard" />
        <div className="flex justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
        </div>
      </div>
    );
  }

  return (
    <div>
      <Header title="Dashboard" />

      <div className="p-6">
        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-500">Total Projects</p>
                  <p className="text-3xl font-bold text-gray-900">{stats.totalProjects}</p>
                  <p className="text-xs text-green-600 mt-1">{stats.activeProjects} active</p>
                </div>
                <div className="p-3 bg-blue-100 rounded-full">
                  <FolderKanban className="h-6 w-6 text-blue-600" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-500">Total Tasks</p>
                  <p className="text-3xl font-bold text-gray-900">{stats.totalTasks}</p>
                  <p className="text-xs text-gray-500 mt-1">
                    {stats.completedTasks} completed, {stats.inProgressTasks} in progress
                  </p>
                </div>
                <div className="p-3 bg-green-100 rounded-full">
                  <CheckSquare className="h-6 w-6 text-green-600" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-500">Teams</p>
                  <p className="text-3xl font-bold text-gray-900">{stats.totalTeams}</p>
                </div>
                <div className="p-3 bg-purple-100 rounded-full">
                  <Users className="h-6 w-6 text-purple-600" />
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="p-6">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-gray-500">Overdue Tasks</p>
                  <p className="text-3xl font-bold text-gray-900">{stats.overdueTasks}</p>
                  {stats.overdueTasks > 0 && (
                    <p className="text-xs text-red-600 mt-1">Needs attention</p>
                  )}
                </div>
                <div className="p-3 bg-red-100 rounded-full">
                  <AlertTriangle className="h-6 w-6 text-red-600" />
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Charts Section */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
          {/* Project Status Chart */}
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Project Status</CardTitle>
            </CardHeader>
            <CardContent>
              {projectStatusData.length > 0 ? (
                <ResponsiveContainer width="100%" height={200}>
                  <PieChart>
                    <Pie
                      data={projectStatusData}
                      cx="50%"
                      cy="50%"
                      innerRadius={40}
                      outerRadius={70}
                      paddingAngle={2}
                      dataKey="value"
                    >
                      {projectStatusData.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={entry.color} />
                      ))}
                    </Pie>
                    <Tooltip />
                    <Legend />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="h-[200px] flex items-center justify-center text-gray-500 text-sm">
                  No project data
                </div>
              )}
            </CardContent>
          </Card>

          {/* Task Status Chart */}
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Task Status</CardTitle>
            </CardHeader>
            <CardContent>
              {taskStatusData.length > 0 ? (
                <ResponsiveContainer width="100%" height={200}>
                  <PieChart>
                    <Pie
                      data={taskStatusData}
                      cx="50%"
                      cy="50%"
                      innerRadius={40}
                      outerRadius={70}
                      paddingAngle={2}
                      dataKey="value"
                    >
                      {taskStatusData.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={entry.color} />
                      ))}
                    </Pie>
                    <Tooltip />
                    <Legend />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="h-[200px] flex items-center justify-center text-gray-500 text-sm">
                  No task data
                </div>
              )}
            </CardContent>
          </Card>

          {/* Task Priority Chart */}
          <Card>
            <CardHeader>
              <CardTitle className="text-base">Tasks by Priority</CardTitle>
            </CardHeader>
            <CardContent>
              <ResponsiveContainer width="100%" height={200}>
                <BarChart data={taskPriorityData}>
                  <XAxis dataKey="name" tick={{ fontSize: 12 }} />
                  <YAxis allowDecimals={false} tick={{ fontSize: 12 }} />
                  <Tooltip />
                  <Bar dataKey="value" fill="#3B82F6" radius={[4, 4, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
            </CardContent>
          </Card>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Recent Projects */}
          <Card>
            <CardHeader className="flex flex-row items-center justify-between">
              <CardTitle>Recent Projects</CardTitle>
              <Link
                href="/projects"
                className="text-sm text-blue-600 hover:underline flex items-center"
              >
                View all
                <ArrowRight className="h-4 w-4 ml-1" />
              </Link>
            </CardHeader>
            <CardContent>
              {recentProjects.length === 0 ? (
                <p className="text-sm text-gray-500 text-center py-4">No projects yet</p>
              ) : (
                <div className="space-y-4">
                  {recentProjects.map((project) => (
                    <Link
                      key={project.id}
                      href={`/projects/${project.id}`}
                      className="flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 transition-colors"
                    >
                      <div className="flex items-center gap-3 min-w-0">
                        <div className="p-2 bg-gray-100 rounded-lg flex-shrink-0">
                          <FolderKanban className="h-4 w-4 text-gray-600" />
                        </div>
                        <div className="min-w-0">
                          <p className="font-medium text-sm truncate">{project.name}</p>
                          <p className="text-xs text-gray-500 truncate">
                            {project.description || 'No description'}
                          </p>
                        </div>
                      </div>
                      <Badge className={`${statusColors[project.status]} flex-shrink-0 ml-2`}>
                        {project.status}
                      </Badge>
                    </Link>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>

          {/* Urgent Tasks */}
          <Card>
            <CardHeader className="flex flex-row items-center justify-between">
              <CardTitle>Urgent Tasks</CardTitle>
              <Link
                href="/tasks"
                className="text-sm text-blue-600 hover:underline flex items-center"
              >
                View all
                <ArrowRight className="h-4 w-4 ml-1" />
              </Link>
            </CardHeader>
            <CardContent>
              {urgentTasks.length === 0 ? (
                <div className="text-center py-8">
                  <TrendingUp className="h-8 w-8 text-green-500 mx-auto mb-2" />
                  <p className="text-sm text-gray-500">No urgent tasks</p>
                </div>
              ) : (
                <div className="space-y-4">
                  {urgentTasks.map((task) => (
                    <div
                      key={task.id}
                      className="flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 transition-colors"
                    >
                      <div className="flex items-center gap-3 min-w-0">
                        <div
                          className={`p-2 rounded-lg flex-shrink-0 ${
                            task.priority === 'Critical' ? 'bg-red-100' : 'bg-orange-100'
                          }`}
                        >
                          <Clock
                            className={`h-4 w-4 ${
                              task.priority === 'Critical' ? 'text-red-600' : 'text-orange-600'
                            }`}
                          />
                        </div>
                        <div className="min-w-0">
                          <p className="font-medium text-sm truncate">{task.title}</p>
                          <div className="flex items-center gap-2 mt-1">
                            <Badge className={taskStatusColors[task.status]}>{task.status}</Badge>
                            {task.due_date && (
                              <span className="text-xs text-gray-500">
                                Due {formatDate(task.due_date)}
                              </span>
                            )}
                          </div>
                        </div>
                      </div>
                      <Badge className={`${priorityColors[task.priority]} flex-shrink-0 ml-2`}>
                        {task.priority}
                      </Badge>
                    </div>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
        </div>

        {/* Task Completion Progress */}
        <Card className="mt-6">
          <CardHeader>
            <CardTitle>Task Progress</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div>
                <div className="flex justify-between text-sm mb-2">
                  <span className="text-gray-600">Overall Completion</span>
                  <span className="font-medium">
                    {stats.totalTasks > 0
                      ? Math.round((stats.completedTasks / stats.totalTasks) * 100)
                      : 0}
                    %
                  </span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-3">
                  <div
                    className="bg-green-600 h-3 rounded-full transition-all"
                    style={{
                      width: `${
                        stats.totalTasks > 0 ? (stats.completedTasks / stats.totalTasks) * 100 : 0
                      }%`,
                    }}
                  />
                </div>
              </div>

              <div className="grid grid-cols-2 md:grid-cols-5 gap-4 pt-4">
                <div className="text-center">
                  <div className="text-2xl font-bold text-gray-900">
                    {tasks.filter((t) => t.status === 'Todo').length}
                  </div>
                  <div className="text-xs text-gray-500">To Do</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-blue-600">{stats.inProgressTasks}</div>
                  <div className="text-xs text-gray-500">In Progress</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-purple-600">
                    {tasks.filter((t) => t.status === 'Review').length}
                  </div>
                  <div className="text-xs text-gray-500">Review</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-green-600">{stats.completedTasks}</div>
                  <div className="text-xs text-gray-500">Done</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-red-600">
                    {tasks.filter((t) => t.status === 'Blocked').length}
                  </div>
                  <div className="text-xs text-gray-500">Blocked</div>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
