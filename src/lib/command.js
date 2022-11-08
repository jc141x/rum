import { invoke } from '@tauri-apps/api/tauri';

class Command {
  async run(mod, command, ...args) {
    console.debug(`Invoke command ${mod}.${command} with ${JSON.stringify([...args])}`);
    return await invoke(`${mod}_${command}`, ...args);
  }

  async library(command, ...args) {
    return await this.run('library', command, ...args);
  }

  async config(command, ...args) {
    return await this.run('config', command, ...args);
  }

  async misc(command, ...args) {
    return await this.run('misc', command, ...args);
  }
}

export default new Command();
