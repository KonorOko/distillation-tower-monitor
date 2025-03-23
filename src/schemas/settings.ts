import { z } from "zod";

export const settingsSchema = z.object({
  initialAddress: z.coerce.number().min(0, "Too low").max(500, "Too high"),
  usbPort: z
    .string()
    .min(1, "USB port is required")
    .optional()
    .or(z.literal("")),
  count: z.number().int().min(1, "Must be at least 1"),
  timeout: z.number().min(100, "Too short").max(10000, "Too long"),
  baudrate: z.number().min(9600).max(115200),
  unitId: z.number().int().min(0).max(255),
});
