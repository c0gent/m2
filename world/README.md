## Notes

### TODO

- Drastically reduce the number of steps it takes to translate between sim and
  win.
  - Keep a stamp (a wrapping int) which tracks the 'generation' of the
    simulation. Incrementing the generation signals that the vertices need to
    be rebuilt from scratch. This rebuilding process should be broken down as
    finely as possible. Markers for each object may need to be used to
    indicate a pending removal. Don't spend too much time on this.