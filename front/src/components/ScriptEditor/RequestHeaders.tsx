import React, { useState, useEffect } from 'react';
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Plus, X } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"

type Header = {
  id: string;
  name: string;
  value: string;
};

interface RequestHeadersProps {
  headers: Header[];
  onHeadersChange: (headers: Header[]) => void;
}

const defaultHeaders = [
  { name: 'Content-Type', value: 'application/json' },
  { name: 'Accept', value: 'application/json' },
  { name: 'Authorization', value: 'Bearer ' },
  { name: 'User-Agent', value: 'MyApp/1.0' },
];

const RequestHeaders: React.FC<RequestHeadersProps> = ({ headers, onHeadersChange }) => {
  const [editingHeaders, setEditingHeaders] = useState<Header[]>(headers);
  const [selectedDefault, setSelectedDefault] = useState<string>('');

  useEffect(() => {
    setEditingHeaders(headers);
  }, [headers]);

  const handleChange = (id: string, field: 'name' | 'value', newValue: string) => {
    const updatedHeaders = editingHeaders.map(header =>
      header.id === id ? { ...header, [field]: newValue } : header
    );
    setEditingHeaders(updatedHeaders);
    onHeadersChange(updatedHeaders);
  };

  const handleAdd = (name: string = '', value: string = '') => {
    const newHeader = { id: Date.now().toString(), name, value };
    const updatedHeaders = [...editingHeaders, newHeader];
    setEditingHeaders(updatedHeaders);
    onHeadersChange(updatedHeaders);
    setSelectedDefault('');
  };

  const handleAddDefault = (headerName: string) => {
    const defaultHeader = defaultHeaders.find(h => `${h.name} ${h.value}` === headerName);
    if (defaultHeader) {
      handleAdd(defaultHeader.name, defaultHeader.value);
    }
  };

  const handleRemove = (id: string) => {
    const updatedHeaders = editingHeaders.filter(header => header.id !== id);
    setEditingHeaders(updatedHeaders);
    onHeadersChange(updatedHeaders);
  };

  return (
    <div className="space-y-2">
      {editingHeaders.map(header => (
        <div key={header.id} className="flex items-center space-x-2">
          <Input
            value={header.name}
            onChange={(e) => handleChange(header.id, 'name', e.target.value)}
            placeholder="Header Name"
            className="flex-1"
          />
          <Input
            value={header.value}
            onChange={(e) => handleChange(header.id, 'value', e.target.value)}
            placeholder="Header Value"
            className="flex-1"
          />
          <Button variant="ghost" size="icon" onClick={() => handleRemove(header.id)}>
            <X className="h-4 w-4" />
          </Button>
        </div>
      ))}
      <div className="flex space-x-2">
        <Select value={selectedDefault} onValueChange={handleAddDefault}>
          <SelectTrigger className="w-[200px]">
            <SelectValue placeholder="Add default header" />
          </SelectTrigger>
          <SelectContent>
            {defaultHeaders.map(header => (
              <SelectItem key={`${header.name} ${header.value}`} value={`${header.name} ${header.value}`}>
                {`${header.name} ${header.value}`}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Button variant="outline" onClick={() => handleAdd()}>
          <Plus className="h-4 w-4 mr-2" /> Add Custom Header
        </Button>
      </div>
    </div>
  );
};

export default RequestHeaders;