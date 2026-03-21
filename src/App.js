import React from 'react';
import RecordingStudio from './front/js/pages/RecordingStudio';

// Stub out platform context so RecordingStudio doesn't crash without Flask
const StubContext = React.createContext({ store: {}, actions: {} });

// Patch the appContext import at runtime
window.__SPX_STUDIO__ = true;

function App() {
  return (
    <StubContext.Provider value={{ store: { user: { subscription_tier: 'studio' } }, actions: {} }}>
      <RecordingStudio user={{ subscription_tier: 'studio', tier: 'studio' }} />
    </StubContext.Provider>
  );
}

export default App;
