import { motion } from "framer-motion";
import { DistillationPlate } from "./distillation-plate";

export function DistillationTower({ plates }: { plates: number }) {
  const maxPlatesBeforeScaling = 5;
  const scale =
    plates <= maxPlatesBeforeScaling ? 1 : maxPlatesBeforeScaling / plates;

  return (
    <div className="flex h-full flex-col-reverse items-center justify-center gap-1 overflow-hidden">
      {[...Array(plates)].map((_, index) => (
        <motion.div
          key={plates - index}
          layout="position"
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{
            duration: 0.1,
          }}
        >
          <DistillationPlate index={index} scale={scale} />
        </motion.div>
      ))}
    </div>
  );
}
