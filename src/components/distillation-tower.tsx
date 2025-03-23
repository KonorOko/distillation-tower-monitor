import { motion } from "framer-motion";
import { DistillationPlate } from "./distillation-plate";

const animationProps = {
  initial: { opacity: 0, scale: 0.8 },
  animate: { opacity: 1, scale: 1 },
  transition: {
    type: "spring",
    stiffness: 300,
    damping: 30,
    duration: 0.3,
  },
};

export function DistillationTower({ plates }: { plates: number }) {
  return (
    <div className="flex h-full flex-col-reverse items-center justify-center gap-1 overflow-auto">
      {[...Array(plates)].map((_, index) => (
        <motion.div
          key={plates - index}
          layout={"position"}
          {...animationProps}
        >
          <DistillationPlate index={index} />
        </motion.div>
      ))}
    </div>
  );
}
