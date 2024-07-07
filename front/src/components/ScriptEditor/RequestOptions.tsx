import React from "react";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { CurlOptions } from "@/store/slices/curlSlice";

interface RequestOptionsProps {
  options: CurlOptions;
  onOptionsChange: (options: CurlOptions) => void;
}

const RequestOptions: React.FC<RequestOptionsProps> = ({ options, onOptionsChange }) => {
  const handleChange = (key: keyof CurlOptions, value: any) => {
    onOptionsChange({ ...options, [key]: value });
  };

  return (
    <div className="space-y-4" onSubmit={() => {}}>
      <div className="grid grid-cols-2 gap-4">
        <div className="flex items-center space-x-2">
          <Switch id="verbose" checked={options.verbose} onCheckedChange={(checked: boolean) => handleChange("verbose", checked)} />
          <Label htmlFor="verbose">Verbose</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="insecure" checked={options.insecure} onCheckedChange={(checked: boolean) => handleChange("insecure", checked)} />
          <Label htmlFor="insecure">Insecure</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="followRedirects" checked={options.followRedirects} onCheckedChange={(checked: boolean) => handleChange("followRedirects", checked)} />
          <Label htmlFor="followRedirects">Follow Redirects</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="compressed" checked={options.compressed} onCheckedChange={(checked: boolean) => handleChange("compressed", checked)} />
          <Label htmlFor="compressed">Compressed</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="fail" checked={options.fail} onCheckedChange={(checked: boolean) => handleChange("fail", checked)} />
          <Label htmlFor="fail">Fail on Error</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="ipv4Only" checked={options.ipv4Only} onCheckedChange={(checked: boolean) => handleChange("ipv4Only", checked)} />
          <Label htmlFor="ipv4Only">IPv4 Only</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="ipv6Only" checked={options.ipv6Only} onCheckedChange={(checked: boolean) => handleChange("ipv6Only", checked)} />
          <Label htmlFor="ipv6Only">IPv6 Only</Label>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="flex items-center space-x-2">
          <Label htmlFor="maxRedirects" className="min-w-[120px]">
            Max Redirects
          </Label>
          <Input
            id="maxRedirects"
            type="number"
            value={options.maxRedirects || ""}
            onChange={(e) => handleChange("maxRedirects", e.target.value === "" ? undefined : parseInt(e.target.value))}
            className="w-24"
            min={0}
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="timeout" className="min-w-[120px]">
            Timeout (s)
          </Label>
          <Input
            id="timeout"
            type="number"
            value={options.timeout || ""}
            onChange={(e) => handleChange("timeout", e.target.value === "" ? undefined : parseInt(e.target.value))}
            className="w-24"
            min={0}
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="connectTimeout" className="min-w-[120px]">
            Connect Timeout (s)
          </Label>
          <Input
            id="connectTimeout"
            type="number"
            value={options.connectTimeout || ""}
            onChange={(e) => handleChange("connectTimeout", e.target.value === "" ? undefined : parseInt(e.target.value))}
            className="w-24"
            min={0}
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="maxTime" className="min-w-[120px]">
            Max Time (s)
          </Label>
          <Input
            id="maxTime"
            type="number"
            value={options.maxTime || ""}
            onChange={(e) => handleChange("maxTime", e.target.value === "" ? undefined : parseInt(e.target.value))}
            className="w-24"
            min={0}
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="retry" className="min-w-[120px]">
            Retry Count
          </Label>
          <Input
            id="retry"
            type="number"
            value={options.retry || ""}
            onChange={(e) => handleChange("retry", e.target.value === "" ? undefined : parseInt(e.target.value))}
            className="w-24"
            min={0}
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="retryDelay" className="min-w-[120px]">
            Retry Delay (s)
          </Label>
          <Input
            id="retryDelay"
            type="number"
            value={options.retryDelay || ""}
            onChange={(e) => handleChange("retryDelay", e.target.value === "" ? undefined : parseInt(e.target.value))}
            className="w-24"
            min={0}
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="rateLimit" className="min-w-[120px]">
            Rate Limit (B/s)
          </Label>
          <Input
            id="rateLimit"
            type="number"
            value={options.rateLimit || ""}
            onChange={(e) => handleChange("rateLimit", e.target.value === "" ? undefined : parseInt(e.target.value))}
            className="w-24"
            min={0}
          />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="flex items-center space-x-2">
          <Label htmlFor="proxy" className="min-w-[120px]">
            Proxy
          </Label>
          <Input
            id="proxy"
            type="text"
            value={options.proxy || ""}
            onChange={(e) => handleChange("proxy", e.target.value)}
            className="flex-1"
            placeholder="e.g., http://proxy.example.com:8080"
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="interface" className="min-w-[120px]">
            Interface
          </Label>
          <Input
            id="interface"
            type="text"
            value={options.interface || ""}
            onChange={(e) => handleChange("interface", e.target.value)}
            className="flex-1"
            placeholder="e.g., eth0"
          />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="flex items-center space-x-2">
          <Label htmlFor="cert" className="min-w-[120px]">
            Certificate
          </Label>
          <Input
            id="cert"
            type="text"
            value={options.cert || ""}
            onChange={(e) => handleChange("cert", e.target.value)}
            className="flex-1"
            placeholder="Path to certificate"
          />
        </div>
        <div className="flex items-center space-x-2">
          <Label htmlFor="key" className="min-w-[120px]">
            Key
          </Label>
          <Input
            id="key"
            type="text"
            value={options.key || ""}
            onChange={(e) => handleChange("key", e.target.value)}
            className="flex-1"
            placeholder="Path to key"
            autoComplete="path-to-key"
          />
        </div>
      </div>

      <div className="flex items-center space-x-2">
        <Label htmlFor="keyPassword" className="min-w-[120px]">
          Key Password
        </Label>
        <form className="flex-1">
          <Input className="hidden" autoComplete="username" />
          <Input
            id="keyPassword"
            type="password"
            value={options.keyPassword || ""}
            onChange={(e) => handleChange("keyPassword", e.target.value)}
            className="w-full"
            placeholder="Password for the key"
            autoComplete="current-password"
          />
        </form>
      </div>

      <div className="flex items-center space-x-2">
        <Label htmlFor="outputFile" className="min-w-[120px]">
          Output File
        </Label>
        <Input
          id="outputFile"
          type="text"
          value={options.outputFile || ""}
          onChange={(e) => handleChange("outputFile", e.target.value)}
          className="flex-1"
          placeholder="Path to output file"
        />
      </div>

      <div className="flex items-center space-x-2">
        <Label htmlFor="dnsServers" className="min-w-[120px]">
          DNS Servers
        </Label>
        <Input
          id="dnsServers"
          type="text"
          value={options.dnsServers ? options.dnsServers.join(", ") : ""}
          onChange={(e) =>
            handleChange(
              "dnsServers",
              e.target.value.split(",").map((s) => s.trim())
            )
          }
          className="flex-1"
          placeholder="Comma-separated list of DNS servers"
        />
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="flex items-center space-x-2">
          <Switch id="timeNamelookup" checked={options.timeNamelookup} onCheckedChange={(checked: boolean) => handleChange("timeNamelookup", checked)} />
          <Label htmlFor="timeNamelookup">Time Namelookup</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="timeConnect" checked={options.timeConnect} onCheckedChange={(checked: boolean) => handleChange("timeConnect", checked)} />
          <Label htmlFor="timeConnect">Time Connect</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="timeAppconnect" checked={options.timeAppconnect} onCheckedChange={(checked: boolean) => handleChange("timeAppconnect", checked)} />
          <Label htmlFor="timeAppconnect">Time Appconnect</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="timePretransfer" checked={options.timePretransfer} onCheckedChange={(checked: boolean) => handleChange("timePretransfer", checked)} />
          <Label htmlFor="timePretransfer">Time Pretransfer</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch
            id="timeStarttransfer"
            checked={options.timeStarttransfer}
            onCheckedChange={(checked: boolean) => handleChange("timeStarttransfer", checked)}
          />
          <Label htmlFor="timeStarttransfer">Time Starttransfer</Label>
        </div>
        <div className="flex items-center space-x-2">
          <Switch id="timeTotal" checked={options.timeTotal} onCheckedChange={(checked: boolean) => handleChange("timeTotal", checked)} />
          <Label htmlFor="timeTotal">Time Total</Label>
        </div>
      </div>
    </div>
  );
};

export default RequestOptions;
