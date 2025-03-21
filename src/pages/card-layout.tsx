import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { cn } from "@/lib/utils";

export function CardLayout({
  className,
  title,
  description,
  children,
}: {
  className?: string;
  title: string;
  description: string;
  children: React.ReactNode;
}) {
  return (
    <Card className={cn(className, "h-full w-full")}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent className="h-[calc(100%-90px)] border-t pb-4">
        {children}
      </CardContent>
    </Card>
  );
}
