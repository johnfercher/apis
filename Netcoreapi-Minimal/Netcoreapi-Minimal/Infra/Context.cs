using Microsoft.EntityFrameworkCore;

namespace Netcoreapi_Minimal.Infra
{
    public class Context : DbContext
    {
        public Context(DbContextOptions<Context> options) : base(options) {}
        public DbSet<Order> Orders { get; set; }
    }
}
