import { invoke } from '@tauri-apps/api/tauri'

export async function getReqsMarkdown() {
  try {
    return await invoke('misc_get_reqs_markdown')
  } catch (error) {
    throw error
  }
}

export async function getWikiPage(page) {
  try {
    return await invoke('misc_get_wiki_page', { page })
  } catch (error) {
    throw error
  }
}

export function initBgProcess() {
  invoke('misc_init_bg_process').catch(error => {
    console.error('Failed to initialize background process:', error)
  })
}
