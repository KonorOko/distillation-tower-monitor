import { z } from "zod";

export const formSchema = z.object({
  usbPort: z.string(),
  baudrate: z
    .enum(["9600", "19200", "38400", "57600", "115200"])
    .transform(Number),
  temperatureTop: z.coerce.number().min(0, "Too low").max(500, "Too high"),
  temperatureBottom: z.coerce.number().min(0, "Too low").max(500, "Too high"),
  unitId: z.coerce.number().int().min(0).max(255),
  timeout: z.coerce.number().min(100).max(10000),
});

export const settingsSchema = z.object({
  usbPort: z.string().min(1, "USB port is required"),
  count: z.number().int().min(1, "Must be at least 1"),
  timeout: z.number().min(100, "Too short").max(10000, "Too long"),
  baudrate: z.number(),
  unitId: z.number().int().min(0).max(255),
  numberPlates: z.number().int().min(1, "At least 1 plate"),
  temperatureAddress: z.object({
    top: z.number().min(0, "Too low").max(500, "Too high"),
    bottom: z.number().min(0, "Too low").max(500, "Too high"),
  }),
});
