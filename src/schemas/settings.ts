import { z } from "zod";

export const formSchema = z.object({
  usbPort: z.string(),
  baudrate: z.number().min(9600).max(115200),
  initialAddress: z.coerce.number().min(0, "Too low").max(500, "Too high"),
  count: z.coerce.number().min(1, "Too low").max(500, "Too high"),
  unitId: z.coerce.number().int().min(0).max(255),
  timeout: z.coerce.number().min(100).max(10000),
});

export const settingsSchema = z.object({
  usbPort: z.string().min(1, "USB port is required"),
  count: z.number().int().min(1, "Must be at least 1"),
  timeout: z.number().min(100, "Too short").max(10000, "Too long"),
  baudrate: z.number().min(9600).max(115200),
  unitId: z.number().int().min(0).max(255),
  numberPlates: z.number().int().min(1, "At least 1 plate"),
});
