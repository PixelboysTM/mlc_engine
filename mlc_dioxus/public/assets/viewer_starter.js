import init from '/mlc_viewer.js'

    init().catch((error) => {
      if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
        throw error;
      }
    });