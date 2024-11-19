/* tslint:disable */
/* eslint-disable */
/**
 * A question, it's answers and a correct answer
 */
export class Question {
  free(): void;
  /**
   * Returns the question's name
   * @returns {string}
   */
  get_name(): string;
  /**
   * Returns the question answers
   * @returns {(string)[]}
   */
  get_answers(): (string)[];
  /**
   * Given an answer, checks if that is the correct one
   * @param {string} chosen
   * @returns {boolean}
   */
  is_correct(chosen: string): boolean;
  /**
   * Gets the correct answer
   * @returns {string}
   */
  get_correct_answer(): string;
}
/**
 * The Main Driver for generating new questions for the day
 */
export class TriviaGenerator {
  free(): void;
  /**
   * Creates a new Trivia set from the cached JSON
   * @returns {TriviaGenerator | undefined}
   */
  static new(): TriviaGenerator | undefined;
  /**
   * Generates a new TriviaGenerator by  reading the existing files
   * @returns {TriviaGenerator | undefined}
   */
  static generate(): TriviaGenerator | undefined;
  /**
   * Get's today's question
   * @returns {Question}
   */
  get_question(): Question;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_triviagenerator_free: (a: number, b: number) => void;
  readonly triviagenerator_new: () => number;
  readonly triviagenerator_generate: () => number;
  readonly triviagenerator_get_question: (a: number) => number;
  readonly __wbg_question_free: (a: number, b: number) => void;
  readonly question_get_name: (a: number, b: number) => void;
  readonly question_get_answers: (a: number, b: number) => void;
  readonly question_is_correct: (a: number, b: number, c: number) => number;
  readonly question_get_correct_answer: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
