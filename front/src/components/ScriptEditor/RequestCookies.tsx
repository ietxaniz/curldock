import React, { useState, useEffect } from 'react';
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Plus, X } from 'lucide-react';

type Cookie = {
  id: string;
  name: string;
  value: string;
};

interface RequestCookiesProps {
  cookies: Cookie[];
  onCookiesChange: (cookies: Cookie[]) => void;
}

const RequestCookies: React.FC<RequestCookiesProps> = ({ cookies, onCookiesChange }) => {
  const [editingCookies, setEditingCookies] = useState<Cookie[]>(cookies);

  useEffect(() => {
    setEditingCookies(cookies);
  }, [cookies]);

  const handleChange = (id: string, field: 'name' | 'value', newValue: string) => {
    const updatedCookies = editingCookies.map(cookie =>
      cookie.id === id ? { ...cookie, [field]: newValue } : cookie
    );
    setEditingCookies(updatedCookies);
    onCookiesChange(updatedCookies);
  };

  const handleAdd = () => {
    const newCookie = { id: Date.now().toString(), name: '', value: '' };
    const updatedCookies = [...editingCookies, newCookie];
    setEditingCookies(updatedCookies);
    onCookiesChange(updatedCookies);
  };

  const handleRemove = (id: string) => {
    const updatedCookies = editingCookies.filter(cookie => cookie.id !== id);
    setEditingCookies(updatedCookies);
    onCookiesChange(updatedCookies);
  };

  return (
    <div className="space-y-2">
      {editingCookies.map(cookie => (
        <div key={cookie.id} className="flex items-center space-x-2">
          <Input
            value={cookie.name}
            onChange={(e) => handleChange(cookie.id, 'name', e.target.value)}
            placeholder="Cookie Name"
            className="flex-1"
          />
          <Input
            value={cookie.value}
            onChange={(e) => handleChange(cookie.id, 'value', e.target.value)}
            placeholder="Cookie Value"
            className="flex-1"
          />
          <Button variant="ghost" size="icon" onClick={() => handleRemove(cookie.id)}>
            <X className="h-4 w-4" />
          </Button>
        </div>
      ))}
      <Button variant="outline" onClick={handleAdd}>
        <Plus className="h-4 w-4 mr-2" /> Add Cookie
      </Button>
    </div>
  );
};

export default RequestCookies;