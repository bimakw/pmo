// User types
export type UserRole = 'Admin' | 'Manager' | 'Member';

export interface User {
  id: string;
  email: string;
  name: string;
  role: UserRole;
  avatar_url?: string;
  created_at: string;
  updated_at: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

// Project types
export type ProjectStatus = 'Planning' | 'Active' | 'OnHold' | 'Completed' | 'Cancelled';
export type Priority = 'Low' | 'Medium' | 'High' | 'Critical';

export interface Project {
  id: string;
  name: string;
  description?: string;
  status: ProjectStatus;
  priority: Priority;
  start_date?: string;
  end_date?: string;
  budget?: string;
  owner_id: string;
  created_at: string;
  updated_at: string;
}

// Task types
export type TaskStatus = 'Todo' | 'inprogress' | 'Review' | 'Done' | 'Blocked';

export interface Task {
  id: string;
  project_id: string;
  milestone_id?: string;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: Priority;
  assignee_id?: string;
  due_date?: string;
  estimated_hours?: number;
  actual_hours?: number;
  created_at: string;
  updated_at: string;
}

// Team types
export type TeamMemberRole = 'Lead' | 'Member';

export interface Team {
  id: string;
  name: string;
  description?: string;
  lead_id?: string;
  created_at: string;
  updated_at: string;
}

export interface TeamMember {
  id: string;
  team_id: string;
  user_id: string;
  role: TeamMemberRole;
  joined_at: string;
}

// Milestone types
export interface Milestone {
  id: string;
  project_id: string;
  name: string;
  description?: string;
  due_date?: string;
  completed: boolean;
  created_at: string;
  updated_at: string;
}

// Time Log
export interface TimeLog {
  id: string;
  task_id: string;
  user_id: string;
  user_name?: string;
  task_name?: string;
  project_name?: string;
  description?: string;
  hours: number;
  date: string;
  created_at: string;
}

// Activity Log
export type ActivityAction =
  | 'created'
  | 'updated'
  | 'deleted'
  | 'status_changed'
  | 'assigned'
  | 'commented';

export type EntityType = 'project' | 'task' | 'team' | 'milestone' | 'comment';

export interface ActivityLog {
  id: string;
  user_id?: string;
  user_name?: string;
  project_id?: string;
  project_name?: string;
  action: ActivityAction;
  entity_type: EntityType;
  entity_id: string;
  entity_name?: string;
  details?: Record<string, unknown>;
  created_at: string;
}

// API Response
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
}
