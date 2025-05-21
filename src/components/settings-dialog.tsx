import { logger } from "@/adapters/tauri";
import { commands } from "@/bindings";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useSettings } from "@/hooks/useSettings";
import { settingsSchema } from "@/schemas/settings";
import { SettingsType } from "@/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { Save, X } from "lucide-react";
import { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";
import { Button } from "./ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "./ui/form";
import { Input } from "./ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";

export function SettingsDialog({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useState(false);
  const { settings, saveSettings } = useSettings();
  const form = useForm<z.infer<typeof settingsSchema>>({
    resolver: zodResolver(settingsSchema),
    defaultValues: {
      count: 2,
    },
  });

  const [usbPorts, setUsbPorts] = useState<string[]>([]);

  async function onSubmit(values: z.infer<typeof settingsSchema>) {
    const { usbPort, baudrate, initialAddress, count, timeout, unitId } =
      values;
    console.log(values);
    try {
      const newSettings: Partial<SettingsType> = {
        modbus: {
          usbPort: usbPort ?? "",
          baudrate,
          initialAddress,
          timeout,
          unitId,
          count,
        },
      };
      await saveSettings(newSettings);
      setOpen(false);
    } catch (error) {
      toast.error("Error on save");
      logger.error("Error on save: " + error);
    }
  }

  useEffect(() => {
    if (!open) return;
    console.log("Settings:", settings);
    form.setValue("usbPort", settings.modbus.usbPort);
    form.setValue("baudrate", settings.modbus.baudrate);
    form.setValue("initialAddress", settings.modbus.initialAddress);
    form.setValue("timeout", settings.modbus.timeout);
    form.setValue("unitId", settings.modbus.unitId);
    form.setValue("count", settings.modbus.count);

    commands.availablePorts().then((response) => {
      if (response.status !== "ok") {
        toast.error("Error on fetch ports");
        logger.error("Error on fetch ports: " + response.error);
        return;
      }

      const ports = response.data;
      if (ports.length === 0) {
        setUsbPorts([]);
        form.setValue("usbPort", "");
      }
      setUsbPorts(ports);
    });
  }, [open]);

  const handleCancel = () => {
    setOpen(false);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Configuration</DialogTitle>
          <DialogDescription>
            Configure the modbus connection parameters
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
            <Tabs className="w-full" defaultValue="connection">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="connection">Connection</TabsTrigger>
                <TabsTrigger value="advanced" disabled>
                  Advanced
                </TabsTrigger>
              </TabsList>
              <TabsContent value="connection" className="space-y-4 pt-4">
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  <FormField
                    control={form.control}
                    name="usbPort"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>USB Port</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          value={field.value}
                        >
                          <FormControl>
                            <SelectTrigger className="w-[200px]">
                              <SelectValue placeholder="Select a port" />
                            </SelectTrigger>
                          </FormControl>
                          <SelectContent>
                            {usbPorts.length === 0 && (
                              <SelectItem value={" "} disabled>
                                No ports found.
                              </SelectItem>
                            )}
                            {usbPorts.map((port) => (
                              <SelectItem key={port} value={port}>
                                {port}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </Select>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="baudrate"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Baudrate</FormLabel>
                        <FormControl>
                          <Input
                            className="w-[200px]"
                            placeholder="Enter baudrate"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="timeout"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Timeout</FormLabel>
                        <FormControl>
                          <Input
                            className="w-[200px]"
                            placeholder="Enter timeout"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="unitId"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Unit ID</FormLabel>
                        <FormControl>
                          <Input
                            className="w-[200px]"
                            placeholder="Enter unit ID"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="initialAddress"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Initial Address</FormLabel>
                        <FormControl>
                          <Input
                            className="w-[200px]"
                            placeholder="Enter initial address"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <FormField
                    control={form.control}
                    name="count"
                    disabled
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Count</FormLabel>
                        <FormControl>
                          <Input
                            className="w-[200px] select-none"
                            placeholder="Enter count"
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </div>
              </TabsContent>
            </Tabs>
            <DialogFooter className="mt-5">
              <Button variant="outline" type="button" onClick={handleCancel}>
                <X className="mr-2 h-4 w-4" />
                Cancel
              </Button>
              <Button type="submit">
                <Save className="mr-2 h-4 w-4" />
                Save
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
}
