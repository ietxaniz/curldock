import { useState } from 'react';
import { Editor, OnChange } from '@monaco-editor/react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import RequestHeaders from './RequestHeaders';

type Header = {
  id: string;
  name: string;
  value: string;
};

const ScriptEditor = () => {
  const [method, setMethod] = useState('GET');
  const [url, setUrl] = useState('');
  const [headers, setHeaders] = useState<Header[]>([]);
  const [bodyContent, setBodyContent] = useState('');

  const handleBodyChange: OnChange = (value, _event) => {
    setBodyContent(value || '');
  };

  const handleSend = () => {
    console.log('Sending request:', { 
      method, 
      url, 
      headers: headers.reduce((acc, h) => ({ ...acc, [h.name]: h.value }), {}),
      bodyContent 
    });
    // Implement actual request sending logic here
  };

  return (
    <div className="p-4 space-y-4">
      <div className="flex space-x-2">
        <Select value={method} onValueChange={setMethod}>
          <SelectTrigger className="w-[100px]">
            <SelectValue placeholder="Method" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="GET">GET</SelectItem>
            <SelectItem value="POST">POST</SelectItem>
            <SelectItem value="PUT">PUT</SelectItem>
            <SelectItem value="DELETE">DELETE</SelectItem>
          </SelectContent>
        </Select>
        <Input 
          type="text" 
          placeholder="Enter request URL" 
          value={url} 
          onChange={(e) => setUrl(e.target.value)}
          className="flex-grow"
        />
        <Button onClick={handleSend} variant="secondary">Send</Button>
      </div>
      
      <div className="space-y-4">
        <div className="border border-gray-200 rounded-md p-4">
          <h3 className="mb-2 font-medium">Headers</h3>
          <RequestHeaders headers={headers} onHeadersChange={setHeaders} />
        </div>
        
        <div className="border border-gray-200 rounded-md">
          <h3 className="px-4 py-2 bg-gray-100 font-medium">Body</h3>
          <Editor
            height="300px"
            defaultLanguage="json"
            value={bodyContent}
            onChange={handleBodyChange}
            options={{
              minimap: { enabled: false },
              lineNumbers: 'on',
              scrollBeyondLastLine: false,
              automaticLayout: true,
            }}
          />
        </div>
      </div>
    </div>
  );
};

export default ScriptEditor;