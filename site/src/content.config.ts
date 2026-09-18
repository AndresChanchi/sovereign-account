import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

const ubiquitousLanguage = z
  .object({
    id: z.string(),
    title: z.string(),
    domain: z.string(),
    type: z.enum(['concept', 'reference', 'index', 'overview']),
    status: z.enum(['draft', 'review', 'stable', 'deprecated']),
    chapter: z.literal('ubiquitous-language').optional(),
    'chapter-number': z.string().optional(),
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
  })
  .refine(
    (data) =>
      data.type === 'index' ||
      (data.chapter === 'ubiquitous-language' && typeof data['chapter-number'] === 'string'),
    { message: 'chapter and chapter-number are required for non-index entries' },
  );

const formalLaws = z
  .object({
    id: z.string(),
    title: z.string(),
    domain: z.string(),
    type: z.union([z.literal('law-set'), z.literal('index'), z.literal('reference')]).optional(),
    status: z.enum(['draft', 'review', 'stable', 'deprecated']),
    chapter: z.literal('formal-laws').optional(),
    'law-id': z.string().optional(),
    laws: z.array(z.string()).optional(),
    'depends-on': z.array(z.string()).default([]),
    related: z.array(z.string()).default([]),
    'formal-laws': z.array(z.string()).default([]),
    verification: z
      .object({
        dafny: z.array(z.string()).default([]),
      })
      .optional(),
  })
  .refine(
    (data) =>
      data.type === 'index' ||
      data.type === 'reference' ||
      (data.type === 'law-set' &&
        data.chapter === 'formal-laws' &&
        typeof data['law-id'] === 'string' &&
        Array.isArray(data.laws)),
    { message: 'law-set entries require chapter, law-id and laws' },
  );

export const collections = {
  'ubiquitous-language': defineCollection({
    loader: glob({
      pattern: '**/*.md',
      base: './src/content/docs/domain/account/ubiquitous-language',
    }),
    schema: ubiquitousLanguage,
  }),
  'formal-laws': defineCollection({
    loader: glob({
      pattern: '**/*.md',
      base: './src/content/docs/domain/account/formal-laws',
    }),
    schema: formalLaws,
  }),
};
