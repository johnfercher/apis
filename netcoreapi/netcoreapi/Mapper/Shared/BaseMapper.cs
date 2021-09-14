using System.Collections.Generic;

namespace netcoreapi.Mapper.Shared
{
    public abstract class BaseMapper<TEntry, TResponse>
          where TEntry : class
          where TResponse : class
    {
        public abstract TResponse Map(TEntry entry);

        public virtual IEnumerable<TResponse> Map(IEnumerable<TEntry> entries)
        {
            foreach (var entry in entries ?? new List<TEntry>())
                yield return Map(entry);
        }
    }
}
