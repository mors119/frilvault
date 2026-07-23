export type AutoSaveStatus = 'editing' | 'saving' | 'saved' | 'failed' | 'conflict';

export function draftFingerprint(content: string, tagsText: string): string {
  return `${content.trim()}\u0000${tagsText.trim()}`;
}

export interface AutoSaveController {
  reset(persistedFingerprint: string): void;
  schedule(fingerprint: string, revision: number): void;
  flush(): Promise<void>;
  cancel(): void;
  startComposition(): void;
  endComposition(): void;
}

export class DebouncedAutoSave implements AutoSaveController {
  private timer: ReturnType<typeof setTimeout> | undefined;

  private lastPersistedFingerprint = '';

  private pendingFingerprint = '';

  private pendingRevision = 0;

  private activeSave: Promise<void> | undefined;

  private saveQueued = false;

  private composing = false;

  private canceled = false;

  public constructor(
    private readonly debounceMs: number,
    private readonly onStatusChange: (status: AutoSaveStatus) => void,
    private readonly persist: (revision: number) => Promise<void>,
  ) {}

  public reset(fingerprint: string): void {
    this.clearTimer();
    this.canceled = false;
    this.composing = false;
    this.saveQueued = false;
    this.pendingFingerprint = fingerprint;
    this.pendingRevision = 0;
    this.lastPersistedFingerprint = fingerprint;
  }

  public schedule(fingerprint: string, revision: number): void {
    this.pendingFingerprint = fingerprint;
    this.pendingRevision = revision;

    if (fingerprint === this.lastPersistedFingerprint) {
      if (!this.activeSave) {
        this.onStatusChange('saved');
      }
      return;
    }

    this.onStatusChange('editing');
    this.clearTimer();

    if (this.composing) {
      return;
    }

    this.timer = setTimeout(() => {
      void this.flush();
    }, this.debounceMs);
  }

  public async flush(): Promise<void> {
    this.clearTimer();

    if (this.composing || this.canceled) {
      return;
    }

    if (this.pendingFingerprint === this.lastPersistedFingerprint) {
      this.onStatusChange('saved');
      return;
    }

    if (this.activeSave) {
      this.saveQueued = true;
      await this.activeSave;
      return;
    }

    this.activeSave = this.runSaveLoop();

    try {
      await this.activeSave;
    } finally {
      this.activeSave = undefined;
    }
  }

  public cancel(): void {
    this.clearTimer();
    this.canceled = true;
    this.saveQueued = false;
  }

  public startComposition(): void {
    this.composing = true;
    this.clearTimer();
  }

  public endComposition(): void {
    this.composing = false;

    if (this.pendingFingerprint === this.lastPersistedFingerprint || this.canceled) {
      return;
    }

    this.onStatusChange('editing');
    this.clearTimer();
    this.timer = setTimeout(() => {
      void this.flush();
    }, this.debounceMs);
  }

  private async runSaveLoop(): Promise<void> {
    while (
      !this.canceled &&
      !this.composing &&
      this.pendingFingerprint !== this.lastPersistedFingerprint
    ) {
      const revision = this.pendingRevision;
      const fingerprint = this.pendingFingerprint;
      this.saveQueued = false;
      this.onStatusChange('saving');

      try {
        await this.persist(revision);
      } catch {
        if (!this.canceled && revision === this.pendingRevision) {
          this.onStatusChange('failed');
        }

        return;
      }

      this.lastPersistedFingerprint = fingerprint;

      if (this.pendingFingerprint === this.lastPersistedFingerprint) {
        this.onStatusChange('saved');
        return;
      }
    }

    if (!this.canceled && this.pendingFingerprint === this.lastPersistedFingerprint) {
      this.onStatusChange('saved');
    }
  }

  private clearTimer(): void {
    if (this.timer) {
      clearTimeout(this.timer);
      this.timer = undefined;
    }
  }
}
