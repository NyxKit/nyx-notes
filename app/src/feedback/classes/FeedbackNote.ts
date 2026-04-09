import Note from '@/notes/classes/Note'

export default class FeedbackNote extends Note {
  constructor(data: unknown) {
    super(data)
  }

  get feedbackType(): string {
    return this.meta.feedback_type ?? 'feedback'
  }

  get appLocation(): string {
    return this.meta.app_location ?? ''
  }

  get storagePath(): string {
    return this.meta.storage_path ?? ''
  }

  get consoleOutput(): string {
    return this.meta.console_output ?? ''
  }

  get interactionTrail(): string | null | undefined {
    return this.meta.interaction_trail ?? null
  }

  get submissionContext(): {
    app_location: string
    storage_path: string
    console_output: string
    interaction_trail: string | null | undefined
  } {
    return {
      app_location: this.appLocation,
      storage_path: this.storagePath,
      console_output: this.consoleOutput,
      interaction_trail: this.interactionTrail,
    }
  }
}
