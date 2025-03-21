import { MAX_DATA_LENGTH } from "@/constants";
import { ColumnDataEntry } from "@/types";
import { formatTime } from "@/lib/utils";

export function formatTempPerTime(columnData: ColumnDataEntry[]) {
  if (!columnData || columnData.length === 0) return [];
  const lastData = columnData.slice(-MAX_DATA_LENGTH);
  let initialDate = lastData[0].timestamp;

  let formatedData = lastData.map((entry) => {
    const transcurredTime = entry.timestamp - initialDate;
    const formattedTime = formatTime(transcurredTime);
    return {
      time: formattedTime,
      ...Object.fromEntries(
        entry.temperatures.map((temp, index) => [`plate${index + 1}`, temp]),
      ),
    };
  });

  return formatedData;
}

export function formatYvsX(columnData: ColumnDataEntry[]) {
  let lastEntry = columnData.slice(-1);
  if (!lastEntry || lastEntry.length === 0) return [];
  let allCompNone = lastEntry[0].compositions.every(
    (comp) => !comp.x_1 && !comp.y_1,
  );
  if (allCompNone) return [];

  return lastEntry.flatMap((entry) =>
    entry.compositions.map((comp) => ({
      x: comp.x_1,
      y: comp.y_1,
    })),
  );
}

export function formatXYvsTemp(columnData: ColumnDataEntry[]) {
  if (!columnData || columnData.length === 0) return [];
  let current = columnData.slice(-1)[0];
  let data = [];
  for (let i = 0; i < current.temperatures.length; i++) {
    let newEntry = [
      {
        temp: current.temperatures[i].toFixed(2),
        x: current.compositions[i].x_1,
      },
      {
        temp: current.temperatures[i].toFixed(2),
        x: current.compositions[i].y_1,
      },
    ];

    data.push(newEntry);
  }

  return data.reverse();
}
