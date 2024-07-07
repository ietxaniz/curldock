import { useEffect, useRef, useState } from "react";
import { useGetCurlItemByFileId, useUpdateCurlItem } from "@/store/hooks/useCurl"
import { HttpMethod } from "@/store/slices/curlSlice";

export type Header = {
  id: string;
  name: string;
  value: string;
};

export const useScriptEditorData = (fileId: number) => {
  const [initialized, setInitialized] = useState(false);
  const getCurlItem = useGetCurlItemByFileId();
  const updateCurlItem = useUpdateCurlItem();
  const [method, setMethod] = useState<HttpMethod>(HttpMethod.GET);
  const [url, setUrl] = useState('');
  const [headers, setHeaders] = useState<Header[]>([]);
  const [bodyContent, setBodyContent] = useState('');

  const exitCallbackRef = useRef(() => {});

  exitCallbackRef.current = () => {
    if (initialized) {
      console.log(`close ${fileId} - ${url}`);
      updateCurlItem({
        fileId: fileId,
        script: {
          method: method,
          url: url,
          headers: headers.map(h => [h.name, h.value]),
          data: bodyContent,
          options: {
            verbose: true,
            insecure: false,
          }
        }
      });
    }
  };


  useEffect(() => {
    const curlItem = getCurlItem(fileId);
    if (curlItem && curlItem.script) {
      console.log(curlItem);
      const { method, url, headers, data } = curlItem.script;
      setMethod(method);
      setUrl(url);
      setHeaders(headers.map((h, index) => ({ id: index.toString(), name: h[0], value: h[1] })));
      setBodyContent(data || '');
      setInitialized(true);
    }
    console.log(`open ${fileId}`);

    return () => { exitCallbackRef.current(); }

  }, [fileId]);

  return { method, setMethod, url, setUrl, headers, setHeaders, bodyContent, setBodyContent };
}

