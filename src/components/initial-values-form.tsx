import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useData } from "@/hooks/useData";
import { useVariables } from "@/hooks/useVariables";
import { cn } from "@/lib/utils";
import { formSchema } from "@/schemas/settings";
import { Info } from "lucide-react";
import { toast } from "sonner";

type FormValues = z.infer<typeof formSchema>;

interface InitialValuesFormProps {
  className?: string;
  defaultValues?: Partial<FormValues>;
}

export function InitialValuesForm({
  className,
  defaultValues = {},
}: InitialValuesFormProps) {
  const connected = useData((state) => state.connected);
  const initialMass = useVariables((state) => state.initialMass);
  const initialComposition = useVariables((state) => state.initialComposition);
  const setInitialMass = useVariables((state) => state.setInitialMass);
  const setInitialComposition = useVariables(
    (state) => state.setInitialComposition,
  );

  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues,
  });

  const handleSubmit = async (values: FormValues) => {
    setInitialMass(values.initialMass ?? 0);
    setInitialComposition(values.initialComposition ?? 0);
    toast.success("Initial values set");
  };

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(handleSubmit)}
        className={cn("space-y-2", className)}
      >
        <div className="grid w-full grid-cols-1 gap-4 md:grid-cols-2">
          <FormField
            control={form.control}
            name="initialMass"
            render={({ field }) => (
              <FormItem>
                <FormLabel className="flex items-center gap-2">
                  Initial mass (g)
                  <Info className="size-3 self-start" />
                </FormLabel>
                <FormControl>
                  <Input
                    placeholder="Ej: 100"
                    {...field}
                    className="max-w-full"
                    disabled={connected === "file"}
                  />
                </FormControl>
              </FormItem>
            )}
          />

          <FormField
            control={form.control}
            name="initialComposition"
            render={({ field }) => (
              <FormItem>
                <FormLabel className="flex items-center gap-2">
                  Composition (%m/m)
                  <Info className="size-3 self-start" />
                </FormLabel>
                <FormControl>
                  <Input
                    placeholder="Ej: 0.89"
                    {...field}
                    className="w-full max-w-full"
                    disabled={connected === "file"}
                  />
                </FormControl>
              </FormItem>
            )}
          />
        </div>
        <Button
          type="submit"
          className="w-full"
          disabled={connected === "file"}
        >
          Set Variables
        </Button>
      </form>
    </Form>
  );
}
