class Command {
  async run(endpoint, command, ...args) {
    console.debug(`Calling API endpoint ${endpoint}/${command} with ${JSON.stringify([...args])}`);
    const response = await fetch(`/api/${endpoint}/${command}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ command, args })
    });
    if (!response.ok) {
      throw new Error(`API request failed with status ${response.status}`);
    }
    return await response.json();
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
