'use client';

import { useState, useRef } from 'react';
import { cn } from '@/lib/utils';
import type { Attachment } from '@/types';
import { attachmentsApi } from '@/lib/api';
import { Upload, File, Image, Trash2, Download, X } from 'lucide-react';
import { Button } from './button';

interface FileUploaderProps {
  taskId: string;
  attachments: Attachment[];
  onAttachmentsChange: (attachments: Attachment[]) => void;
  className?: string;
}

const formatFileSize = (bytes: number) => {
  const KB = 1024;
  const MB = KB * 1024;
  if (bytes >= MB) {
    return `${(bytes / MB).toFixed(2)} MB`;
  } else if (bytes >= KB) {
    return `${(bytes / KB).toFixed(2)} KB`;
  }
  return `${bytes} B`;
};

const getFileIcon = (contentType: string) => {
  if (contentType.startsWith('image/')) {
    return <Image className="h-4 w-4 text-blue-500" />;
  }
  return <File className="h-4 w-4 text-gray-500" />;
};

export function FileUploader({ taskId, attachments, onAttachmentsChange, className }: FileUploaderProps) {
  const [uploading, setUploading] = useState(false);
  const [dragActive, setDragActive] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleUpload = async (files: FileList | null) => {
    if (!files || files.length === 0) return;

    setUploading(true);
    try {
      const newAttachments: Attachment[] = [];
      for (const file of Array.from(files)) {
        const response = await attachmentsApi.upload(taskId, file);
        if (response.data) {
          newAttachments.push(response.data);
        }
      }
      onAttachmentsChange([...attachments, ...newAttachments]);
    } catch (error) {
      console.error('Failed to upload file:', error);
    } finally {
      setUploading(false);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await attachmentsApi.delete(id);
      onAttachmentsChange(attachments.filter((a) => a.id !== id));
    } catch (error) {
      console.error('Failed to delete attachment:', error);
    }
  };

  const handleDrag = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === 'dragenter' || e.type === 'dragover') {
      setDragActive(true);
    } else if (e.type === 'dragleave') {
      setDragActive(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    handleUpload(e.dataTransfer.files);
  };

  return (
    <div className={cn('space-y-3', className)}>
      {/* Upload area */}
      <div
        className={cn(
          'border-2 border-dashed rounded-lg p-4 text-center transition-colors cursor-pointer',
          dragActive ? 'border-blue-500 bg-blue-50' : 'border-gray-200 hover:border-gray-300',
          uploading && 'opacity-50 pointer-events-none'
        )}
        onDragEnter={handleDrag}
        onDragLeave={handleDrag}
        onDragOver={handleDrag}
        onDrop={handleDrop}
        onClick={() => fileInputRef.current?.click()}
      >
        <input
          ref={fileInputRef}
          type="file"
          multiple
          className="hidden"
          onChange={(e) => handleUpload(e.target.files)}
        />
        <Upload className="h-8 w-8 mx-auto text-gray-400 mb-2" />
        <p className="text-sm text-gray-600">
          {uploading ? 'Uploading...' : 'Drop files here or click to upload'}
        </p>
        <p className="text-xs text-gray-400 mt-1">Max 10MB per file</p>
      </div>

      {/* Attachments list */}
      {attachments.length > 0 && (
        <div className="space-y-2">
          {attachments.map((attachment) => (
            <div
              key={attachment.id}
              className="flex items-center justify-between p-2 bg-gray-50 rounded-lg"
            >
              <div className="flex items-center gap-2 min-w-0">
                {getFileIcon(attachment.content_type)}
                <div className="min-w-0">
                  <p className="text-sm font-medium text-gray-900 truncate">
                    {attachment.original_filename}
                  </p>
                  <p className="text-xs text-gray-500">{formatFileSize(attachment.size_bytes)}</p>
                </div>
              </div>
              <div className="flex items-center gap-1 ml-2">
                <a
                  href={attachmentsApi.download(attachment.id)}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="p-1.5 text-gray-400 hover:text-blue-600 hover:bg-blue-50 rounded transition-colors"
                >
                  <Download className="h-4 w-4" />
                </a>
                <button
                  type="button"
                  onClick={() => handleDelete(attachment.id)}
                  className="p-1.5 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded transition-colors"
                >
                  <Trash2 className="h-4 w-4" />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
