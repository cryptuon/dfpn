export const MODALITY_MAP: Record<number, string> = {
  1: 'Image Authenticity',
  2: 'Video Authenticity',
  4: 'Audio Authenticity',
  8: 'Face Manipulation',
  16: 'Voice Cloning',
  32: 'Generated Content',
}

export const MODALITY_COLORS: Record<number, string> = {
  1: 'bg-blue-500/20 text-blue-400',
  2: 'bg-purple-500/20 text-purple-400',
  4: 'bg-green-500/20 text-green-400',
  8: 'bg-orange-500/20 text-orange-400',
  16: 'bg-pink-500/20 text-pink-400',
  32: 'bg-cyan-500/20 text-cyan-400',
}

export function parseModalities(bitfield: number): { bit: number; label: string; color: string }[] {
  const result: { bit: number; label: string; color: string }[] = []
  for (const [bit, label] of Object.entries(MODALITY_MAP)) {
    const b = Number(bit)
    if (bitfield & b) {
      result.push({ bit: b, label, color: MODALITY_COLORS[b] })
    }
  }
  return result
}

export function modalityOptions() {
  return Object.entries(MODALITY_MAP).map(([bit, label]) => ({
    value: Number(bit),
    label,
  }))
}
