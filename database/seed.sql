-- PMO Database Seed Data
-- Sample data for development and testing

-- Insert sample users (password: "Password123" hashed with argon2)
INSERT INTO users (id, email, password_hash, name, role) VALUES
    ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'admin@pmo.local', '$argon2id$v=19$m=65536,t=3,p=4$rZntpJ3B2Ac4ahee3G/a9w$WXBcSo0oX3mY8h23fmIYF6KKHSRNhD51VDV9MqMuwF8', 'Admin User', 'admin'),
    ('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'manager@pmo.local', '$argon2id$v=19$m=65536,t=3,p=4$rZntpJ3B2Ac4ahee3G/a9w$WXBcSo0oX3mY8h23fmIYF6KKHSRNhD51VDV9MqMuwF8', 'Project Manager', 'manager'),
    ('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', 'dev1@pmo.local', '$argon2id$v=19$m=65536,t=3,p=4$rZntpJ3B2Ac4ahee3G/a9w$WXBcSo0oX3mY8h23fmIYF6KKHSRNhD51VDV9MqMuwF8', 'Developer One', 'member'),
    ('d0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', 'dev2@pmo.local', '$argon2id$v=19$m=65536,t=3,p=4$rZntpJ3B2Ac4ahee3G/a9w$WXBcSo0oX3mY8h23fmIYF6KKHSRNhD51VDV9MqMuwF8', 'Developer Two', 'member');

-- Insert sample teams
INSERT INTO teams (id, name, description, lead_id) VALUES
    ('e0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15', 'Engineering', 'Core engineering team', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12'),
    ('f0eebc99-9c0b-4ef8-bb6d-6bb9bd380a16', 'Design', 'UI/UX design team', NULL);

-- Insert team members
INSERT INTO team_members (team_id, user_id, role) VALUES
    ('e0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'lead'),
    ('e0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15', 'c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', 'member'),
    ('e0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15', 'd0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', 'member');

-- Insert sample projects
INSERT INTO projects (id, name, description, status, priority, start_date, end_date, budget, owner_id) VALUES
    ('10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'Website Redesign', 'Complete redesign of company website', 'active', 'high', '2024-01-01', '2024-06-30', 50000.00, 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12'),
    ('20eebc99-9c0b-4ef8-bb6d-6bb9bd380a18', 'Mobile App Development', 'New mobile application for customers', 'planning', 'critical', '2024-03-01', '2024-12-31', 150000.00, 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12'),
    ('30eebc99-9c0b-4ef8-bb6d-6bb9bd380a19', 'Database Migration', 'Migrate legacy database to PostgreSQL', 'completed', 'medium', '2023-10-01', '2024-01-15', 25000.00, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11');

-- Insert project members
INSERT INTO project_members (project_id, user_id, role) VALUES
    ('10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'Project Manager'),
    ('10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', 'Frontend Developer'),
    ('10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'd0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', 'Backend Developer'),
    ('20eebc99-9c0b-4ef8-bb6d-6bb9bd380a18', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'Project Manager'),
    ('20eebc99-9c0b-4ef8-bb6d-6bb9bd380a18', 'c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', 'Mobile Developer');

-- Insert milestones
INSERT INTO milestones (id, project_id, name, description, due_date, completed) VALUES
    ('40eebc99-9c0b-4ef8-bb6d-6bb9bd380a20', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'Design Phase', 'Complete all UI/UX designs', '2024-02-15', true),
    ('50eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'Development Phase', 'Implement frontend and backend', '2024-05-01', false),
    ('60eebc99-9c0b-4ef8-bb6d-6bb9bd380a22', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'Launch', 'Go live with new website', '2024-06-30', false);

-- Insert tasks
INSERT INTO tasks (id, project_id, milestone_id, title, description, status, priority, assignee_id, due_date, estimated_hours) VALUES
    ('70eebc99-9c0b-4ef8-bb6d-6bb9bd380a23', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', '50eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', 'Setup Next.js project', 'Initialize Next.js with TypeScript', 'done', 'high', 'c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', '2024-02-20', 8),
    ('80eebc99-9c0b-4ef8-bb6d-6bb9bd380a24', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', '50eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', 'Implement homepage', 'Build the new homepage design', 'inprogress', 'high', 'c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', '2024-03-01', 24),
    ('90eebc99-9c0b-4ef8-bb6d-6bb9bd380a25', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', '50eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', 'Setup API endpoints', 'Create REST API for the website', 'inprogress', 'high', 'd0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', '2024-03-15', 40),
    ('a1eebc99-9c0b-4ef8-bb6d-6bb9bd380a26', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', '50eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', 'User authentication', 'Implement login/register flow', 'todo', 'critical', 'd0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', '2024-03-20', 32),
    ('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a27', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', '60eebc99-9c0b-4ef8-bb6d-6bb9bd380a22', 'Deploy to production', 'Setup CI/CD and deploy', 'todo', 'medium', NULL, '2024-06-25', 16);

-- Insert task comments
INSERT INTO task_comments (task_id, user_id, content) VALUES
    ('80eebc99-9c0b-4ef8-bb6d-6bb9bd380a24', 'c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', 'Started working on the hero section'),
    ('80eebc99-9c0b-4ef8-bb6d-6bb9bd380a24', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'Looks great! Please also add the testimonials section'),
    ('90eebc99-9c0b-4ef8-bb6d-6bb9bd380a25', 'd0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', 'Using Axum framework for the API');

-- Insert activity logs
INSERT INTO activity_logs (user_id, project_id, action, entity_type, entity_id, details) VALUES
    ('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'created', 'project', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', '{"name": "Website Redesign"}'),
    ('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'status_changed', 'task', '70eebc99-9c0b-4ef8-bb6d-6bb9bd380a23', '{"from": "todo", "to": "done"}'),
    ('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', '10eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'status_changed', 'task', '80eebc99-9c0b-4ef8-bb6d-6bb9bd380a24', '{"from": "todo", "to": "inprogress"}');
