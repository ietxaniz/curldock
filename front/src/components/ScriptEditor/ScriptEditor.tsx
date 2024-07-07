import { useEffect, useState } from 'react';
import { Editor, OnChange } from '@monaco-editor/react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import RequestHeaders from './RequestHeaders';
import RequestCookies from './RequestCookies';
import RequestOptions from './RequestOptions';
import { HttpMethod } from '../../store/slices/curlSlice';
import { useScriptEditorData } from './useScriptEditorData';
import { useHeaderHeight } from '@/store/hooks/useLayout';

interface ScriptEditorProps {
  fileId: number;
}

const ScriptEditor: React.FC<ScriptEditorProps> = ({ fileId }) => {
  const { method, setMethod, url, setUrl, headers, setHeaders, cookies, setCookies, bodyContent, setBodyContent, options, setOptions } = useScriptEditorData(fileId);
  const headerHeight = useHeaderHeight();
  const [remainingHeight, setRemainingHeight] = useState('100vh');

  const handleBodyChange: OnChange = (value, _event) => {
    setBodyContent(value || '');
  };

  const handleSend = async () => {
    console.log('Sending request:', {
      method,
      url,
      headers,
      cookies,
      bodyContent,
      options,
    });
    // Implement actual request sending logic here
  };

  const handleSave = async () => {
    console.log('Saving request:', {
      method,
      url,
      headers,
      cookies,
      bodyContent,
      options,
    });
    // Implement actual request sending logic here
  };

  useEffect(() => {
    const remInPx = parseFloat(getComputedStyle(document.documentElement).fontSize);
    setRemainingHeight(`calc(100vh - ${headerHeight + 1 * remInPx}px)`);
  }, [headerHeight]);

  return (
    <div className="w-full overflow-auto" style={{ height: remainingHeight }}>
      <div className="p-4 space-y-4">
        <div className="flex space-x-2">
          <Select value={method} onValueChange={(value) => setMethod(value as HttpMethod)}>
            <SelectTrigger className="w-[100px]">
              <SelectValue placeholder="Method" />
            </SelectTrigger>
            <SelectContent>
              {Object.values(HttpMethod).map((m) => (
                <SelectItem key={m} value={m}>
                  {m}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <Input type="text" placeholder="Enter request URL" value={url} onChange={(e) => setUrl(e.target.value)} className="flex-grow" />
          <Button onClick={handleSend} variant="secondary">
            Send
          </Button>
          <Button onClick={handleSave} variant="secondary">
            Save
          </Button>
        </div>

        <div className="space-y-4">
          <div className="border border-gray-200 rounded-md p-4">
            <h3 className="mb-2 font-medium">Headers</h3>
            <RequestHeaders headers={headers} onHeadersChange={setHeaders} />
          </div>

          <div className="border border-gray-200 rounded-md p-4">
            <h3 className="mb-2 font-medium">Cookies</h3>
            <RequestCookies cookies={cookies} onCookiesChange={setCookies} />
          </div>

          <div className="border border-gray-200 rounded-md p-4">
            <h3 className="mb-2 font-medium">Body</h3>
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


          <div className="border border-gray-200 rounded-md p-4">
            <h3 className="mb-2 font-medium">Options</h3>
            <RequestOptions options={options} onOptionsChange={setOptions} />
          </div>

        </div>
      </div>
    </div>
  );
};

export default ScriptEditor;