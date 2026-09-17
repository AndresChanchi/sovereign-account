import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

const ubiquitousLanguage = z.object({
  id: z.string(),
  title: z.string(),
  domain: z.string(),
  chapter: z.literal('ubiquitous-language'),
  'chapter-number': z.string(),
  type: z.enum(['concept', 'reference', 'index', 'overview']),
  status: z.enum(['draft', 'review', 'stable', 'deprecated']),
  section: z.string().optional(),
  entities: z.array(z.string()).default([]),
  'value-objects': z.array(z.string()).default([]),
  'depends-on': z.array(z.string()).default([]),
  related: z.array(z.string()).default([]),
  'formal-laws': z.array(z.string()).default([]),
  verification: z
    .object({
      dafny: z.array(z.string()).default([]),
    })
    .optional(),
});

const formalLaws = z.object({
  id: z.string(),
  title: z.string(),
  domain: z.string(),
  chapter: z.literal('formal-laws'),
  type: z.literal('law-set'),
  status: z.enum(['draft', 'review', 'stable', 'deprecated']),
  'law-id': z.string(),
  laws: z.array(z.string()),
  'depends-on': z.array(z.string()).default([]),
  related: z.array(z.string()).default([]),
  'formal-laws': z.array(z.string()).default([]),
  verification: z
    .object({
      dafny: z.array(z.string()).default([]),
    })
    .optional(),
});

export const collections = {
  'ubiquitous-language': defineCollection({
    loader: glob({
      pattern: ['**/*.md', '!**/index.md'],
      base: './src/content/docs/domain/account/ubiquitous-language',
    }),
    schema: ubiquitousLanguage,
  }),
  'formal-laws': defineCollection({
    loader: glob({
      pattern: ['**/*.md', '!**/index.md'],
      base: './src/content/docs/domain/account/formal-laws',
    }),
    schema: formalLaws,
  }),
};
