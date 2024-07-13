import { useEffect, useRef, useState } from "react";
import { useGetCurlItemByFileId, useUpdateCurlItem, useUpdateCurlResult } from "@/store/hooks/useCurl";
import { HttpMethod, CurlOptions, CurlCommand } from "@/store/slices/curlSlice";
import { useGetTreeData } from "@/store/hooks/useFileexplorer";
import { updateScript } from "@/api/updateScript";
import { executeScript } from "../../api/executeScript";

export type Header = {
  id: string;
  name: string;
  value: string;
};

export type Cookie = {
  id: string;
  name: string;
  value: string;
};

export const useScriptEditorData = (fileId: number) => {
  const [initialized, setInitialized] = useState(false);
  const files = useGetTreeData();
  const getCurlItem = useGetCurlItemByFileId();
  const updateCurlItem = useUpdateCurlItem();
  const [method, setMethod] = useState<HttpMethod>(HttpMethod.GET);
  const [url, setUrl] = useState("");
  const [headers, setHeaders] = useState<Header[]>([]);
  const [cookies, setCookies] = useState<Cookie[]>([]);
  const [bodyContent, setBodyContent] = useState("");
  const [options, setOptions] = useState<CurlOptions>({ verbose: false, insecure: false });
  const updateCurlResult = useUpdateCurlResult();

  const curlItem = getCurlItem(fileId);
  const currentCurlItem = useRef(curlItem);
  currentCurlItem.current = curlItem;

  const exitCallbackRef = useRef(() => {});

  exitCallbackRef.current = () => {
    if (initialized) {
      console.log(`close ${fileId} - ${url}`);
      const existingCurlItem = getCurlItem(fileId);
      updateCurlItem({
        fileId: fileId,
        script: {
          method: method,
          url: url,
          headers: headers.map((h) => [h.name, h.value]),
          cookies: cookies.map((c) => [c.name, c.value]),
          data: bodyContent,
          options: options,
        },
        result: existingCurlItem?.result,
      });
    }
  };

  useEffect(() => {
    const curlItem = currentCurlItem.current;
    if (curlItem && curlItem.script) {
      console.log(curlItem);
      const { method, url, headers, cookies, data, options } = curlItem.script;
      setMethod(method);
      setUrl(url);
      setHeaders(headers.map((h, index) => ({ id: index.toString(), name: h[0], value: h[1] })));
      setCookies(cookies?.map((c, index) => ({ id: index.toString(), name: c[0], value: c[1] })) || []);
      setBodyContent(data || "");
      setOptions(options || { verbose: false, insecure: false });
      setInitialized(true);
    }
    console.log(`open ${fileId}`);

    return () => {
      exitCallbackRef.current();
    };
  }, [fileId]);

  const saveCurrentInRedux = exitCallbackRef.current;

  const saveCurrentInBackend = async () => {
    const name = files[fileId].data.name;
    const path = files[fileId].data.path;
    const curlCommand: CurlCommand = {
      method: method,
      url: url,
      headers: headers.map((h) => [h.name, h.value]),
      cookies: cookies.map((c) => [c.name, c.value]),
      data: bodyContent,
      options: options,
    };
    await updateScript({ name, path, curlCommand });
  };

  const callToExecuteScript = async () => {
    const name = files[fileId].data.name;
    const path = files[fileId].data.path;
    await saveCurrentInBackend();
    const response = await executeScript(path, name);
    if (response) {
      updateCurlResult(fileId, response);
    } else {
      // updateCurlResult(fileId, undefined);
    }
  };

  return {
    method,
    setMethod,
    url,
    setUrl,
    headers,
    setHeaders,
    cookies,
    setCookies,
    bodyContent,
    setBodyContent,
    options,
    setOptions,
    saveCurrentInRedux,
    saveCurrentInBackend,
    callToExecuteScript,
  };
};
