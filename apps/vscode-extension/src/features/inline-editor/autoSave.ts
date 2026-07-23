export type AutoSaveStatus = 'editing' | 'saving' | 'saved' | 'failed' | 'conflict';

export function draftFingerprint(content: string, tagsText: string): string {
  return `${content.trim()}\u0000${tagsText.trim()}`;
}

export class DebouncedAutoSave {
  private timer: ReturnType<typeof setTimeout> | undefined;

  private saveGeneration = 0;

  private lastPersistedFingerprint = '';

  public constructor(
    private readonly debounceMs: number,
    private readonly onStatusChange: (status: AutoSaveStatus) => void,
    private readonly persist: (generation: number) => Promise<void>,
  ) {}

  public setPersistedFingerprint(fingerprint: string): void {
    this.lastPersistedFingerprint = fingerprint;
  }

  public schedule(fingerprint: string): void {
    if (fingerprint === this.lastPersistedFingerprint) {
      this.onStatusChange('saved');
      return;
    }

    this.onStatusChange('editing');
    this.clearTimer();
    this.timer = setTimeout(() => {
      void this.flush();
    }, this.debounceMs);
  }

  public async flush(): Promise<void> {
    this.clearTimer();
    const generation = ++this.saveGeneration;
    this.onStatusChange('saving');

    try {
      await this.persist(generation);

      if (generation !== this.saveGeneration) {
        return;
      }

      this.onStatusChange('saved');
    } catch {
      if (generation === this.saveGeneration) {
        this.onStatusChange('failed');
      }
    }
  }

  public cancel(): void {
    this.clearTimer();
    this.saveGeneration += 1;
  }

  public isLatestGeneration(generation: number): boolean {
    return generation === this.saveGeneration;
  }

  private clearTimer(): void {
    if (this.timer) {
      clearTimeout(this.timer);
      this.timer = undefined;
    }
  }
}
