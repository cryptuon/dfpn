import rss from '@astrojs/rss'
import { getCollection } from 'astro:content'
import type { APIContext } from 'astro'

export async function GET(context: APIContext) {
  const posts = (await getCollection('blog', ({ data }) => !data.draft))
    .sort((a, b) => b.data.publishedAt.getTime() - a.data.publishedAt.getTime())

  return rss({
    title: 'DFPN Blog - Deepfake Detection Articles',
    description: 'Technical articles on deepfake detection, AI-generated media verification, decentralized consensus, and the DFPN protocol.',
    site: context.site!.toString(),
    items: posts.map((post) => ({
      title: post.data.title,
      pubDate: post.data.publishedAt,
      description: post.data.description,
      link: `/blog/${post.id}/`,
      categories: post.data.tags,
      author: post.data.author,
    })),
    customData: '<language>en-us</language>',
  })
}
