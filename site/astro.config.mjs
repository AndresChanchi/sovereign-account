import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import { unified } from '@astrojs/markdown-remark';

/**
 * Removes the leading H1 from markdown bodies.
 * The page layout renders its own H1 from the frontmatter title.
 * The raw .md files copied to public/ keep their H1 for standalone consumption.
 */
function remarkStripLeadingH1() {
  return (tree) => {
    if (
      tree.children.length > 0 &&
      tree.children[0].type === 'heading' &&
      tree.children[0].depth === 1
    ) {
      tree.children.shift();
    }
  };
}

/**
 * Rewrites markdown-internal `.md` links to route-style hrefs during HTML render.
 * Keeps the raw .md files (in public/) untouched for LLM consumption.
 */
function rehypeRewriteMdLinks() {
  return (tree) => {
    const walk = (node) => {
      if (node.type === 'element' && node.tagName === 'a') {
        const href = node.properties?.href;
        if (typeof href === 'string' && href.endsWith('.md')) {
          node.properties.href = href.replace(/\.md$/, '');
        }
      }
      if (Array.isArray(node.children)) {
        node.children.forEach(walk);
      }
    };
    walk(tree);
  };
}

export default defineConfig({
  site: 'https://sovereign-account.example',
  output: 'static',
  build: {
    format: 'directory',
  },
  vite: {
    plugins: [tailwindcss()],
  },
  markdown: {
    processor: unified({
      remarkPlugins: [remarkStripLeadingH1],
      rehypePlugins: [rehypeRewriteMdLinks],
    }),
    shikiConfig: {
      theme: 'github-dark',
    },
  },
});
