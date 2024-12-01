self.onmessage = (e) => {
  const { sharedArray } = e.data;
  self.postMessage({ sharedArray });
};
