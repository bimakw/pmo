'use client';

import { useState } from 'react';
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  addDays,
  addMonths,
  subMonths,
  isSameMonth,
  isSameDay,
  isToday,
} from 'date-fns';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { Task, Milestone } from '@/types';

interface CalendarEvent {
  id: string;
  title: string;
  date: string;
  type: 'task' | 'milestone';
  status?: string;
  priority?: string;
}

interface CalendarProps {
  tasks: Task[];
  milestones: Milestone[];
  onDateClick?: (date: Date) => void;
  onEventClick?: (event: CalendarEvent) => void;
}

const priorityColors: Record<string, string> = {
  Low: 'bg-gray-200 text-gray-700',
  Medium: 'bg-blue-200 text-blue-700',
  High: 'bg-orange-200 text-orange-700',
  Critical: 'bg-red-200 text-red-700',
};

export function Calendar({ tasks, milestones, onDateClick, onEventClick }: CalendarProps) {
  const [currentMonth, setCurrentMonth] = useState(new Date());

  const events: CalendarEvent[] = [
    ...tasks
      .filter((task) => task.due_date)
      .map((task) => ({
        id: task.id,
        title: task.title,
        date: task.due_date!,
        type: 'task' as const,
        status: task.status,
        priority: task.priority,
      })),
    ...milestones
      .filter((m) => m.due_date)
      .map((m) => ({
        id: m.id,
        title: m.name,
        date: m.due_date!,
        type: 'milestone' as const,
      })),
  ];

  const getEventsForDate = (date: Date) => {
    return events.filter((event) => isSameDay(new Date(event.date), date));
  };

  const renderHeader = () => {
    return (
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold text-gray-900">
          {format(currentMonth, 'MMMM yyyy')}
        </h2>
        <div className="flex gap-2">
          <button
            onClick={() => setCurrentMonth(subMonths(currentMonth, 1))}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ChevronLeft className="h-5 w-5 text-gray-600" />
          </button>
          <button
            onClick={() => setCurrentMonth(new Date())}
            className="px-3 py-1 text-sm text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
          >
            Today
          </button>
          <button
            onClick={() => setCurrentMonth(addMonths(currentMonth, 1))}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ChevronRight className="h-5 w-5 text-gray-600" />
          </button>
        </div>
      </div>
    );
  };

  const renderDays = () => {
    const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    return (
      <div className="grid grid-cols-7 mb-2">
        {days.map((day) => (
          <div
            key={day}
            className="text-center text-sm font-medium text-gray-500 py-2"
          >
            {day}
          </div>
        ))}
      </div>
    );
  };

  const renderCells = () => {
    const monthStart = startOfMonth(currentMonth);
    const monthEnd = endOfMonth(monthStart);
    const startDate = startOfWeek(monthStart);
    const endDate = endOfWeek(monthEnd);

    const rows = [];
    let days = [];
    let day = startDate;

    while (day <= endDate) {
      for (let i = 0; i < 7; i++) {
        const currentDay = day;
        const dayEvents = getEventsForDate(currentDay);
        const isCurrentMonth = isSameMonth(day, monthStart);

        days.push(
          <div
            key={day.toString()}
            className={cn(
              'min-h-[100px] border border-gray-100 p-1 cursor-pointer transition-colors',
              !isCurrentMonth && 'bg-gray-50',
              isToday(day) && 'bg-blue-50',
              'hover:bg-gray-50'
            )}
            onClick={() => onDateClick?.(currentDay)}
          >
            <div
              className={cn(
                'text-sm font-medium mb-1',
                !isCurrentMonth && 'text-gray-400',
                isToday(day) && 'text-blue-600'
              )}
            >
              {format(day, 'd')}
            </div>
            <div className="space-y-1">
              {dayEvents.slice(0, 3).map((event) => (
                <div
                  key={event.id}
                  onClick={(e) => {
                    e.stopPropagation();
                    onEventClick?.(event);
                  }}
                  className={cn(
                    'text-xs px-1.5 py-0.5 rounded truncate cursor-pointer',
                    event.type === 'milestone'
                      ? 'bg-purple-200 text-purple-700'
                      : priorityColors[event.priority || 'Medium']
                  )}
                  title={event.title}
                >
                  {event.type === 'milestone' && '🎯 '}
                  {event.title}
                </div>
              ))}
              {dayEvents.length > 3 && (
                <div className="text-xs text-gray-500 px-1">
                  +{dayEvents.length - 3} more
                </div>
              )}
            </div>
          </div>
        );
        day = addDays(day, 1);
      }
      rows.push(
        <div key={day.toString()} className="grid grid-cols-7">
          {days}
        </div>
      );
      days = [];
    }
    return <div>{rows}</div>;
  };

  return (
    <div className="bg-white rounded-lg shadow p-4">
      {renderHeader()}
      {renderDays()}
      {renderCells()}
    </div>
  );
}
