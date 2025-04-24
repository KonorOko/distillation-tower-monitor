import { motion } from "framer-motion";
import { DistillationPlate } from "./distillation-plate";

const animationProps = {
  initial: { opacity: 0, scale: 0.8 },
  animate: { opacity: 1, scale: 1 },
  transition: {
    stiffness: 300,
    damping: 30,
    duration: 0.2,
  },
};

export function DistillationTower({ plates }: { plates: number }) {
  const maxPlatesBeforeScaling = 5;
  const scale =
    plates <= maxPlatesBeforeScaling ? 1 : maxPlatesBeforeScaling / plates;

  return (
    <div className="flex h-full flex-col-reverse items-center justify-center gap-1 overflow-hidden">
      {[...Array(plates)].map((_, index) => (
        <motion.div key={plates - index} layout="position" {...animationProps}>
          <DistillationPlate index={index} scale={scale} />
        </motion.div>
      ))}
    </div>
  );
}
