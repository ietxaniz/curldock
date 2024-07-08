import { useEffect, useState } from "react";
import { useGetCurlItemByFileId } from "@/store/hooks/useCurl";
import { Editor } from "@monaco-editor/react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { useHeaderHeight } from "@/store/hooks/useLayout";

interface ScriptResultProps {
  fileId: number;
}

const ScriptResult: React.FC<ScriptResultProps> = ({ fileId }) => {
  const headerHeight = useHeaderHeight();
  const [remainingHeight, setRemainingHeight] = useState("100vh");
  useEffect(() => {
    const remInPx = parseFloat(getComputedStyle(document.documentElement).fontSize);
    setRemainingHeight(`calc(100vh - ${headerHeight + 1 * remInPx}px)`);
  }, [headerHeight]);

  const curlItem = useGetCurlItemByFileId()(fileId);
  const result = curlItem?.result;

  if (!result) {
    return <div className="p-4">No result available. Execute the script to see the results.</div>;
  }

  const formatBody = (body: string) => {
    try {
      return JSON.stringify(JSON.parse(body), null, 2);
    } catch {
      return body;
    }
  };

  return (
    <div className="w-full overflow-auto" style={{ height: remainingHeight }}>
      <div className="p-4 space-y-6 overflow-auto h-full">
        <h2 className="text-2xl font-bold">Script Execution Result</h2>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <h3 className="text-lg font-semibold">Status Code</h3>
            <p>{result.statusCode}</p>
          </div>
          <div>
            <h3 className="text-lg font-semibold">Date</h3>
            <p>{result.date}</p>
          </div>
        </div>

        <div>
          <h3 className="text-lg font-semibold mb-2">Response Headers</h3>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Header</TableHead>
                <TableHead>Value</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {Object.entries(result.responseHeaders).map(([key, value]) => (
                <TableRow key={key}>
                  <TableCell>{key}</TableCell>
                  <TableCell>{value}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>

        <div>
          <h3 className="text-lg font-semibold mb-2">Body</h3>
          <Editor
            height="300px"
            defaultLanguage="json"
            value={formatBody(result.body)}
            options={{
              readOnly: true,
              minimap: { enabled: false },
              scrollBeyondLastLine: false,
              automaticLayout: true,
            }}
          />
        </div>

        <div>
          <h3 className="text-lg font-semibold mb-2">Cookies</h3>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Value</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {Object.entries(result.cookies).map(([key, value]) => (
                <TableRow key={key}>
                  <TableCell>{key}</TableCell>
                  <TableCell>{value}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>

        <div>
          <h3 className="text-lg font-semibold mb-2">Timing Information</h3>
          <Table>
            <TableBody>
              {result.timeNamelookup !== undefined && (
                <TableRow>
                  <TableCell>Name Lookup</TableCell>
                  <TableCell>{result.timeNamelookup} µs</TableCell>
                </TableRow>
              )}
              {result.timeConnect !== undefined && (
                <TableRow>
                  <TableCell>Connect</TableCell>
                  <TableCell>{result.timeConnect} µs</TableCell>
                </TableRow>
              )}
              {result.timeAppconnect !== undefined && (
                <TableRow>
                  <TableCell>App Connect</TableCell>
                  <TableCell>{result.timeAppconnect} µs</TableCell>
                </TableRow>
              )}
              {result.timePretransfer !== undefined && (
                <TableRow>
                  <TableCell>Pretransfer</TableCell>
                  <TableCell>{result.timePretransfer} µs</TableCell>
                </TableRow>
              )}
              {result.timeStarttransfer !== undefined && (
                <TableRow>
                  <TableCell>Start Transfer</TableCell>
                  <TableCell>{result.timeStarttransfer} µs</TableCell>
                </TableRow>
              )}
              {result.timeTotal !== undefined && (
                <TableRow>
                  <TableCell>Total</TableCell>
                  <TableCell>{result.timeTotal} µs</TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
        </div>

        {result.redirectCount !== undefined && (
          <div>
            <h3 className="text-lg font-semibold">Redirect Count</h3>
            <p>{result.redirectCount}</p>
          </div>
        )}

        {result.effectiveUrl && (
          <div>
            <h3 className="text-lg font-semibold">Effective URL</h3>
            <p>{result.effectiveUrl}</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default ScriptResult;
