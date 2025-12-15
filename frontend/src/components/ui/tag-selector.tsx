'use client';

import { useState, useEffect, useRef } from 'react';
import { cn } from '@/lib/utils';
import type { Tag } from '@/types';
import { tagsApi } from '@/lib/api';
import { TagBadge } from './tag-badge';
import { ChevronDown, Plus, X } from 'lucide-react';

interface TagSelectorProps {
  taskId: string;
  selectedTags: Tag[];
  onTagsChange: (tags: Tag[]) => void;
  className?: string;
}

export function TagSelector({ taskId, selectedTags, onTagsChange, className }: TagSelectorProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [allTags, setAllTags] = useState<Tag[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const loadTags = async () => {
      try {
        const response = await tagsApi.list();
        if (response.data) {
          setAllTags(response.data);
        }
      } catch (error) {
        console.error('Failed to load tags:', error);
      }
    };
    loadTags();
  }, []);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const availableTags = allTags.filter(
    (tag) =>
      !selectedTags.find((t) => t.id === tag.id) &&
      tag.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const handleAddTag = async (tag: Tag) => {
    setLoading(true);
    try {
      await tagsApi.addTagToTask(taskId, tag.id);
      onTagsChange([...selectedTags, tag]);
      setSearchQuery('');
    } catch (error) {
      console.error('Failed to add tag:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRemoveTag = async (tagId: string) => {
    setLoading(true);
    try {
      await tagsApi.removeTagFromTask(taskId, tagId);
      onTagsChange(selectedTags.filter((t) => t.id !== tagId));
    } catch (error) {
      console.error('Failed to remove tag:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className={cn('relative', className)} ref={dropdownRef}>
      <div className="flex flex-wrap gap-1 items-center min-h-[32px]">
        {selectedTags.map((tag) => (
          <TagBadge
            key={tag.id}
            tag={tag}
            onRemove={() => handleRemoveTag(tag.id)}
          />
        ))}
        <button
          type="button"
          onClick={() => setIsOpen(!isOpen)}
          disabled={loading}
          className="inline-flex items-center gap-1 px-2 py-0.5 text-xs text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-full transition-colors"
        >
          <Plus className="h-3 w-3" />
          Add tag
          <ChevronDown className={cn('h-3 w-3 transition-transform', isOpen && 'rotate-180')} />
        </button>
      </div>

      {isOpen && (
        <div className="absolute z-10 mt-1 w-64 bg-white border border-gray-200 rounded-lg shadow-lg">
          <div className="p-2 border-b border-gray-100">
            <input
              type="text"
              placeholder="Search tags..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full px-2 py-1 text-sm border border-gray-200 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
            />
          </div>
          <div className="max-h-48 overflow-y-auto p-1">
            {availableTags.length === 0 ? (
              <p className="px-2 py-3 text-sm text-gray-500 text-center">
                {searchQuery ? 'No tags found' : 'All tags assigned'}
              </p>
            ) : (
              availableTags.map((tag) => (
                <button
                  key={tag.id}
                  type="button"
                  onClick={() => handleAddTag(tag)}
                  disabled={loading}
                  className="w-full flex items-center gap-2 px-2 py-1.5 text-sm hover:bg-gray-50 rounded transition-colors"
                >
                  <span
                    className="w-3 h-3 rounded-full flex-shrink-0"
                    style={{ backgroundColor: tag.color }}
                  />
                  <span className="truncate">{tag.name}</span>
                </button>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
}
