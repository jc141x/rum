import { invoke } from '../../node_modules/@tauri-apps/api/tauri';

class Command {
  async run(mod, command, ...args) {
    return await invoke(`${mod}_${command}`, ...args);
  }

  async library(command, ...args) {
    return await this.run('library', command, ...args);
  }

  async config(command, ...args) {
    return await this.run('config', command, ...args);
  }

  async database(command, ...args) {
    return await this.run('database', command, ...args);
  }

  async download(command, ...args) {
    return await this.run('download', command, ...args);
  }

  async misc(command, ...args) {
    return await this.run('misc', command, ...args);
  }
}

export default new Command();
