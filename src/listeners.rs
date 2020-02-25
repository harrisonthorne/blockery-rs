/* jshint esversion: 6, browser: true, devel: true */

const Listeners = {
   blockCountListeners: [],
   invokeBlockCountListeners: |count| {
      self.blockCountListeners.forEach(listener => {
         listener.onBlockCount(count);
      });
   },

   factoryPurchaseListeners: [],
   invokeFactoryPurchaseListeners: |factoryCode, count| {
      self.factoryPurchaseListeners.forEach(listener => {
         listener.onFactoryPurchase(factoryCode, count);
      });
   }
};
