import { AnimatePresence, motion } from "framer-motion";
import { DistillationPlate } from "./distillation-plate";

const animationProps = {
  initial: { opacity: 0, scale: 0.8 },
  animate: { opacity: 1, scale: 1 },
  exit: { opacity: 0, scale: 0.3, transition: { duration: 0.2 } },
  transition: {
    opacity: { duration: 0.2 },
    layout: { type: "spring", stiffness: 300, damping: 30 },
    duration: 0.3,
  },
};

export function DistillationTower({ plates }: { plates: number }) {
  return (
    <div className="flex h-full flex-col-reverse items-center justify-center gap-1 overflow-scroll">
      <AnimatePresence initial={false}>
        {[...Array(plates)].map((_, index) => (
          <motion.div key={plates - index} layout {...animationProps}>
            <DistillationPlate index={index} />
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
