import type { AstroIntegration } from 'astro'
import { readdir, readFile, writeFile, mkdir } from 'node:fs/promises'
import { join } from 'node:path'

/**
 * Astro integration that generates llms-full.txt at build time.
 * Concatenates all blog articles into a single plain-text file
 * for AI models to ingest in one request.
 */
export default function llmsFullIntegration(): AstroIntegration {
  return {
    name: 'llms-full',
    hooks: {
      'astro:build:done': async ({ dir }) => {
        const blogDir = join(process.cwd(), 'src', 'content', 'blog')
        let output = '# DFPN - Deepfake Proof Network - Full Content\n\n'
        output += '> This file contains the full text of all DFPN blog articles for AI model ingestion.\n\n'

        try {
          const files = await readdir(blogDir)
          const mdFiles = files.filter(f => f.endsWith('.md') || f.endsWith('.mdx')).sort()

          for (const file of mdFiles) {
            const content = await readFile(join(blogDir, file), 'utf-8')
            // Strip frontmatter
            const bodyMatch = content.match(/^---[\s\S]*?---\s*(.*)$/s)
            const body = bodyMatch ? bodyMatch[1].trim() : content

            // Extract title from frontmatter
            const titleMatch = content.match(/^title:\s*["']?(.+?)["']?\s*$/m)
            const title = titleMatch ? titleMatch[1] : file.replace(/\.mdx?$/, '')

            output += `---\n\n## ${title}\n\n${body}\n\n`
          }

          // Write to the output directory (dir already points to dist/client/)
          const outDir = new URL('.', dir).pathname
          await writeFile(join(outDir, 'llms-full.txt'), output, 'utf-8')
          console.log('[llms-full] Generated llms-full.txt')
        } catch (e) {
          console.warn('[llms-full] Warning: Could not generate llms-full.txt:', e)
        }
      },
    },
  }
}
