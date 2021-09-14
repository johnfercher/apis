using Microsoft.EntityFrameworkCore;
using netcoreapi.domain.entities;

namespace netcoreapi.infra.memorydb.Db
{
    public class ApiContext : DbContext
    {
        public ApiContext(DbContextOptions<ApiContext> options) : base(options) { }

        public DbSet<Order> Orders { get; set; }
    }
}
