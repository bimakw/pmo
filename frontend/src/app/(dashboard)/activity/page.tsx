'use client';

import { useState, useEffect } from 'react';
import { Header } from '@/components/layout/header';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { projectsApi, activityApi } from '@/lib/api';
import type { Project, ActivityLog, ActivityAction, EntityType } from '@/types';
import {
  Activity,
  FolderKanban,
  CheckSquare,
  Users,
  Target,
  MessageSquare,
  Plus,
  Edit,
  Trash2,
  ArrowRight,
  Filter,
  ChevronDown,
} from 'lucide-react';

const actionIcons: Record<ActivityAction, typeof Plus> = {
  created: Plus,
  updated: Edit,
  deleted: Trash2,
  status_changed: ArrowRight,
  assigned: Users,
  commented: MessageSquare,
};

const actionColors: Record<ActivityAction, string> = {
  created: 'bg-green-100 text-green-600',
  updated: 'bg-blue-100 text-blue-600',
  deleted: 'bg-red-100 text-red-600',
  status_changed: 'bg-purple-100 text-purple-600',
  assigned: 'bg-orange-100 text-orange-600',
  commented: 'bg-gray-100 text-gray-600',
};

const entityIcons: Record<EntityType, typeof FolderKanban> = {
  project: FolderKanban,
  task: CheckSquare,
  team: Users,
  milestone: Target,
  comment: MessageSquare,
};

function getActionText(activity: ActivityLog): string {
  const { action, entity_type, entity_name, details } = activity;

  switch (action) {
    case 'created':
      return `created ${entity_type} "${entity_name}"`;
    case 'updated':
      return `updated ${entity_type} "${entity_name}"`;
    case 'deleted':
      return `deleted ${entity_type} "${entity_name}"`;
    case 'status_changed':
      return `changed status of "${entity_name}" from ${details?.from} to ${details?.to}`;
    case 'assigned':
      return `assigned "${entity_name}" to ${details?.assignee}`;
    case 'commented':
      return `commented on "${entity_name}"`;
    default:
      return `performed action on "${entity_name}"`;
  }
}

function formatTimeAgo(dateString: string): string {
  const date = new Date(dateString);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  const minutes = Math.floor(diff / (1000 * 60));
  const hours = Math.floor(diff / (1000 * 60 * 60));
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  if (minutes < 1) return 'Just now';
  if (minutes < 60) return `${minutes}m ago`;
  if (hours < 24) return `${hours}h ago`;
  if (days < 7) return `${days}d ago`;

  return date.toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'short',
    year: days > 365 ? 'numeric' : undefined,
  });
}

interface ActivityItemProps {
  activity: ActivityLog;
}

function ActivityItem({ activity }: ActivityItemProps) {
  const ActionIcon = actionIcons[activity.action];
  const EntityIcon = entityIcons[activity.entity_type];

  return (
    <div className="flex gap-4 p-4 hover:bg-gray-50 transition-colors">
      <div className={`p-2 rounded-full h-fit ${actionColors[activity.action]}`}>
        <ActionIcon className="h-4 w-4" />
      </div>
      <div className="flex-1 min-w-0">
        <p className="text-sm text-gray-900">
          <span className="font-medium">{activity.user_name}</span>{' '}
          {getActionText(activity)}
        </p>
        <div className="flex items-center gap-2 mt-1">
          {activity.project_name && (
            <Badge variant="secondary" className="text-xs">
              <EntityIcon className="h-3 w-3 mr-1" />
              {activity.project_name}
            </Badge>
          )}
          <span className="text-xs text-gray-500">
            {formatTimeAgo(activity.created_at)}
          </span>
        </div>
      </div>
    </div>
  );
}

export default function ActivityPage() {
  const [activities, setActivities] = useState<ActivityLog[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedProject, setSelectedProject] = useState<string>('all');
  const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null);
  const [showProjectDropdown, setShowProjectDropdown] = useState(false);

  useEffect(() => {
    loadProjects();
  }, []);

  useEffect(() => {
    loadActivities();
  }, [selectedProjectId]);

  const loadProjects = async () => {
    try {
      const projectsRes = await projectsApi.list();
      if (projectsRes.data) setProjects(projectsRes.data);
    } catch (error) {
      console.error('Failed to load projects:', error);
    }
  };

  const loadActivities = async () => {
    setLoading(true);
    try {
      const params = selectedProjectId ? { project_id: selectedProjectId } : undefined;
      const activitiesRes = await activityApi.list(params);
      if (activitiesRes.data) setActivities(activitiesRes.data);
    } catch (error) {
      console.error('Failed to load activities:', error);
    } finally {
      setLoading(false);
    }
  };

  // Activities are already filtered by project_id from API
  const filteredActivities = activities;

  // Group activities by date
  const groupedActivities = filteredActivities.reduce((groups, activity) => {
    const date = new Date(activity.created_at);
    const today = new Date();
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    let key: string;
    if (date.toDateString() === today.toDateString()) {
      key = 'Today';
    } else if (date.toDateString() === yesterday.toDateString()) {
      key = 'Yesterday';
    } else {
      key = date.toLocaleDateString('id-ID', {
        weekday: 'long',
        day: 'numeric',
        month: 'long',
      });
    }

    if (!groups[key]) {
      groups[key] = [];
    }
    groups[key].push(activity);
    return groups;
  }, {} as Record<string, ActivityLog[]>);

  if (loading) {
    return (
      <div>
        <Header title="Activity Feed" />
        <div className="flex justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600" />
        </div>
      </div>
    );
  }

  return (
    <div>
      <Header title="Activity Feed" />

      <div className="p-6">
        <div className="max-w-3xl mx-auto">
          {/* Filter */}
          <div className="mb-6">
            <div className="relative inline-block">
              <button
                onClick={() => setShowProjectDropdown(!showProjectDropdown)}
                className="flex items-center gap-2 px-4 py-2 border rounded-lg hover:bg-gray-50"
              >
                <Filter className="h-4 w-4" />
                <span className="text-sm">
                  {selectedProject === 'all'
                    ? 'All Projects'
                    : selectedProject}
                </span>
                <ChevronDown className="h-4 w-4" />
              </button>

              {showProjectDropdown && (
                <div className="absolute top-full left-0 mt-1 w-64 bg-white border rounded-lg shadow-lg z-10">
                  <button
                    onClick={() => {
                      setSelectedProject('all');
                      setSelectedProjectId(null);
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
                        setSelectedProject(project.name);
                        setSelectedProjectId(project.id);
                        setShowProjectDropdown(false);
                      }}
                      className={`w-full text-left px-4 py-2 hover:bg-gray-50 text-sm ${
                        selectedProjectId === project.id ? 'bg-blue-50 text-blue-600' : ''
                      }`}
                    >
                      {project.name}
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>

          {/* Activity List */}
          <Card>
            <CardHeader className="border-b">
              <div className="flex items-center gap-2">
                <Activity className="h-5 w-5 text-blue-600" />
                <CardTitle>Recent Activity</CardTitle>
              </div>
            </CardHeader>
            <CardContent className="p-0">
              {Object.keys(groupedActivities).length === 0 ? (
                <div className="text-center py-12">
                  <Activity className="h-12 w-12 text-gray-300 mx-auto mb-4" />
                  <p className="text-gray-500">No activity yet</p>
                </div>
              ) : (
                <div>
                  {Object.entries(groupedActivities).map(([date, items]) => (
                    <div key={date}>
                      <div className="px-4 py-2 bg-gray-50 border-b">
                        <p className="text-xs font-medium text-gray-500 uppercase">
                          {date}
                        </p>
                      </div>
                      <div className="divide-y">
                        {items.map((activity) => (
                          <ActivityItem key={activity.id} activity={activity} />
                        ))}
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
