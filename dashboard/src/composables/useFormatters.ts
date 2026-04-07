import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'

dayjs.extend(relativeTime)

const DFPN_DECIMALS = 9

export function formatDfpn(amount: number | bigint): string {
  const num = typeof amount === 'bigint' ? Number(amount) : amount
  const value = num / Math.pow(10, DFPN_DECIMALS)
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(2)}M`
  if (value >= 1_000) return `${(value / 1_000).toFixed(2)}K`
  return value.toFixed(2)
}

export function formatReputation(score: number): string {
  return `${(score / 100).toFixed(1)}%`
}

export function truncateAddress(address: string, chars = 4): string {
  if (address.length <= chars * 2 + 3) return address
  return `${address.slice(0, chars)}...${address.slice(-chars)}`
}

export function formatTimestamp(ts: number): string {
  return dayjs.unix(ts).format('YYYY-MM-DD HH:mm')
}

export function formatRelative(ts: number): string {
  return dayjs.unix(ts).fromNow()
}

export function formatNumber(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}K`
  return n.toLocaleString()
}

export function successRate(completed: number, failed: number): string {
  const total = completed + failed
  if (total === 0) return 'N/A'
  return `${((completed / total) * 100).toFixed(1)}%`
}
