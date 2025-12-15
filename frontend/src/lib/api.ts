import axios from 'axios';
import type {
  ApiResponse,
  AuthResponse,
  User,
  Project,
  Task,
  Team,
  TeamMember,
  Milestone,
  ActivityLog,
  TimeLog,
  Tag,
  Attachment,
} from '@/types';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add token to requests
api.interceptors.request.use((config) => {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
  }
  return config;
});

// Handle 401 responses
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      if (typeof window !== 'undefined') {
        localStorage.removeItem('token');
        localStorage.removeItem('user');
        window.location.href = '/login';
      }
    }
    return Promise.reject(error);
  }
);

// Auth
export const authApi = {
  login: async (email: string, password: string) => {
    const { data } = await api.post<ApiResponse<AuthResponse>>('/auth/login', { email, password });
    return data;
  },
  register: async (email: string, password: string, name: string) => {
    const { data } = await api.post<ApiResponse<User>>('/auth/register', { email, password, name });
    return data;
  },
};

// Projects
export const projectsApi = {
  list: async () => {
    const { data } = await api.get<ApiResponse<Project[]>>('/projects');
    return data;
  },
  get: async (id: string) => {
    const { data } = await api.get<ApiResponse<Project>>(`/projects/${id}`);
    return data;
  },
  create: async (project: Partial<Project>) => {
    const { data } = await api.post<ApiResponse<Project>>('/projects', project);
    return data;
  },
  update: async (id: string, project: Partial<Project>) => {
    const { data } = await api.put<ApiResponse<Project>>(`/projects/${id}`, project);
    return data;
  },
  delete: async (id: string) => {
    const { data } = await api.delete<ApiResponse<void>>(`/projects/${id}`);
    return data;
  },
  getTasks: async (id: string) => {
    const { data } = await api.get<ApiResponse<Task[]>>(`/projects/${id}/tasks`);
    return data;
  },
  getMilestones: async (id: string) => {
    const { data } = await api.get<ApiResponse<Milestone[]>>(`/projects/${id}/milestones`);
    return data;
  },
};

// Tasks
export const tasksApi = {
  list: async () => {
    const { data } = await api.get<ApiResponse<Task[]>>('/tasks');
    return data;
  },
  get: async (id: string) => {
    const { data } = await api.get<ApiResponse<Task>>(`/tasks/${id}`);
    return data;
  },
  create: async (task: Partial<Task>) => {
    const { data } = await api.post<ApiResponse<Task>>('/tasks', task);
    return data;
  },
  update: async (id: string, task: Partial<Task>) => {
    const { data } = await api.put<ApiResponse<Task>>(`/tasks/${id}`, task);
    return data;
  },
  delete: async (id: string) => {
    const { data } = await api.delete<ApiResponse<void>>(`/tasks/${id}`);
    return data;
  },
};

// Teams
export const teamsApi = {
  list: async () => {
    const { data } = await api.get<ApiResponse<Team[]>>('/teams');
    return data;
  },
  get: async (id: string) => {
    const { data } = await api.get<ApiResponse<Team>>(`/teams/${id}`);
    return data;
  },
  create: async (team: Partial<Team>) => {
    const { data } = await api.post<ApiResponse<Team>>('/teams', team);
    return data;
  },
  update: async (id: string, team: Partial<Team>) => {
    const { data } = await api.put<ApiResponse<Team>>(`/teams/${id}`, team);
    return data;
  },
  delete: async (id: string) => {
    const { data } = await api.delete<ApiResponse<void>>(`/teams/${id}`);
    return data;
  },
  getMembers: async (id: string) => {
    const { data } = await api.get<ApiResponse<TeamMember[]>>(`/teams/${id}/members`);
    return data;
  },
  addMember: async (teamId: string, userId: string, role?: string) => {
    const { data } = await api.post<ApiResponse<TeamMember>>(`/teams/${teamId}/members`, { user_id: userId, role });
    return data;
  },
};

// Activity Logs
export const activityApi = {
  list: async (params?: { project_id?: string; limit?: number }) => {
    const { data } = await api.get<ApiResponse<ActivityLog[]>>('/activities', { params });
    return data;
  },
  getByProject: async (projectId: string, limit?: number) => {
    const { data } = await api.get<ApiResponse<ActivityLog[]>>(`/projects/${projectId}/activities`, {
      params: { limit },
    });
    return data;
  },
};

// Time Logs
export const timeLogsApi = {
  list: async (params?: { task_id?: string; user_id?: string; start_date?: string; end_date?: string }) => {
    const { data } = await api.get<ApiResponse<TimeLog[]>>('/time-logs', { params });
    return data;
  },
  getByTask: async (taskId: string) => {
    const { data } = await api.get<ApiResponse<TimeLog[]>>(`/tasks/${taskId}/time-logs`);
    return data;
  },
  create: async (timeLog: { task_id: string; hours: number; date: string; description?: string }) => {
    const { data } = await api.post<ApiResponse<TimeLog>>('/time-logs', timeLog);
    return data;
  },
  update: async (id: string, timeLog: Partial<TimeLog>) => {
    const { data } = await api.put<ApiResponse<TimeLog>>(`/time-logs/${id}`, timeLog);
    return data;
  },
  delete: async (id: string) => {
    const { data } = await api.delete<ApiResponse<void>>(`/time-logs/${id}`);
    return data;
  },
};

// Tags
export const tagsApi = {
  list: async () => {
    const { data } = await api.get<ApiResponse<Tag[]>>('/tags');
    return data;
  },
  get: async (id: string) => {
    const { data } = await api.get<ApiResponse<Tag>>(`/tags/${id}`);
    return data;
  },
  create: async (tag: { name: string; color: string; description?: string }) => {
    const { data } = await api.post<ApiResponse<Tag>>('/tags', tag);
    return data;
  },
  update: async (id: string, tag: Partial<Tag>) => {
    const { data } = await api.put<ApiResponse<Tag>>(`/tags/${id}`, tag);
    return data;
  },
  delete: async (id: string) => {
    const { data } = await api.delete<ApiResponse<void>>(`/tags/${id}`);
    return data;
  },
  getTaskTags: async (taskId: string) => {
    const { data } = await api.get<ApiResponse<Tag[]>>(`/tasks/${taskId}/tags`);
    return data;
  },
  setTaskTags: async (taskId: string, tagIds: string[]) => {
    const { data } = await api.put<ApiResponse<Tag[]>>(`/tasks/${taskId}/tags`, { tag_ids: tagIds });
    return data;
  },
  addTagToTask: async (taskId: string, tagId: string) => {
    const { data } = await api.post<ApiResponse<void>>(`/tasks/${taskId}/tags/${tagId}`);
    return data;
  },
  removeTagFromTask: async (taskId: string, tagId: string) => {
    const { data } = await api.delete<ApiResponse<void>>(`/tasks/${taskId}/tags/${tagId}`);
    return data;
  },
};

// Attachments
export const attachmentsApi = {
  getByTask: async (taskId: string) => {
    const { data } = await api.get<ApiResponse<Attachment[]>>(`/tasks/${taskId}/attachments`);
    return data;
  },
  upload: async (taskId: string, file: File) => {
    const formData = new FormData();
    formData.append('file', file);
    const { data } = await api.post<ApiResponse<Attachment>>(`/tasks/${taskId}/attachments`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });
    return data;
  },
  download: (id: string) => {
    return `${API_URL}/attachments/${id}`;
  },
  delete: async (id: string) => {
    const { data } = await api.delete<ApiResponse<void>>(`/attachments/${id}`);
    return data;
  },
};

export default api;
