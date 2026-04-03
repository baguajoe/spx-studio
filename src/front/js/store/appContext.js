import React from 'react';
export const Context = React.createContext({
  store: { user: { subscription_tier: 'studio', tier: 'studio' } },
  actions: {}
});
