export function formatDuration(totalSecs: number, compact = false): string {
  const secs = Math.max(0, Math.floor(totalSecs));
  const hours = Math.floor(secs / 3600);
  const mins = Math.floor((secs % 3600) / 60);
  const remSecs = secs % 60;

  if (hours > 0) {
    if (compact) {
      return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
    }
    return `${hours}h ${String(mins).padStart(2, "0")}m`;
  }
  if (mins > 0) {
    if (compact) {
      return `${mins}m`;
    }
    return remSecs > 0 && secs < 600 ? `${mins}m ${String(remSecs).padStart(2, "0")}s` : `${mins}m`;
  }
  return `${remSecs}s`;
}

export function formatMinutesAxis(minutes: number): string {
  if (minutes >= 60) {
    const h = Math.floor(minutes / 60);
    return `${h}h`;
  }
  return `${Math.round(minutes)}m`;
}

export function formatStopwatch(totalSecs: number): string {
  const secs = Math.max(0, Math.floor(totalSecs));
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = secs % 60;
  if (h > 0) {
    return `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
  }
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

export function formatDateISO(date: Date): string {
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, "0");
  const d = String(date.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

export function shiftDateISO(dateStr: string, deltaDays: number): string {
  const [y, m, d] = dateStr.split("-").map(Number);
  const dt = new Date(y, (m || 1) - 1, d || 1);
  dt.setDate(dt.getDate() + deltaDays);
  return formatDateISO(dt);
}

export function formatTimestampHm(ts: number | null): string {
  if (!ts) return "暂未记录";
  const dt = new Date(ts * 1000);
  return `${String(dt.getHours()).padStart(2, "0")}:${String(dt.getMinutes()).padStart(2, "0")}`;
}
