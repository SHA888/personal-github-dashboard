module.exports = {
  // Project configuration
  project: {
    name: 'personal-github-dashboard',
    version: '1.0.0'
  },

  // Task configuration
  tasks: {
    // Default task settings
    default: {
      timeout: 30000, // 30 seconds
      retries: 3
    }
  },

  // Logging configuration
  logging: {
    level: 'info',
    format: 'json'
  }
};
