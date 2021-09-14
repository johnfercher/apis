using Microsoft.AspNetCore.Mvc;
using netcoreapi.domain.entities;
using netcoreapi.domain.Interfaces.Services;
using netcoreapi.DTOs;
using netcoreapi.Mapper;

namespace netcoreapi.Controllers
{
    [Route("v1/[controller]")]
    [ApiController]
    public class OrderController : ControllerBase
    {
        private readonly IOrderService _orderService;

        private readonly OrderRequestDtoToOrder _orderRequestDtoToOrder;
        private readonly OrderToOrderResponseDto _orderToOrderResponseDto;

        public OrderController(IOrderService orderService)
        {
            _orderService = orderService;
            _orderRequestDtoToOrder = new OrderRequestDtoToOrder();
            _orderToOrderResponseDto = new OrderToOrderResponseDto();
        }

        // GET: api/<OrderController>
        [HttpGet]
        public IActionResult Get()
        {
            return new OkObjectResult(_orderToOrderResponseDto.Map(_orderService.GetAll()));
        }

        // GET api/<OrderController>/5
        [HttpGet("{id}")]

        public IActionResult GetById(string id)
        {
            var order = _orderToOrderResponseDto.Map(_orderService.GetById(id));
            if (order != null)
            {
                return new OkObjectResult(order);
            }
            return new NotFoundResult();
        }

        // POST api/<OrderController>
        [HttpPost]
        public IActionResult Post([FromBody] OrderRequestDto order)
        {
            var createdOrder = _orderService.Add(_orderRequestDtoToOrder.Map(order));
            return new CreatedResult("/order", _orderToOrderResponseDto.Map(createdOrder));
        }

        // PUT api/<OrderController>/5
        [HttpPut("{id}")]
        public IActionResult Put(string id, [FromBody] OrderRequestDto order)
        {
            var updatedOrder = _orderService.Update(id, _orderRequestDtoToOrder.Map(order));
            if (updatedOrder != null)
            {
                return new OkObjectResult(_orderToOrderResponseDto.Map(updatedOrder));
            }

            return new NotFoundResult();
        }

        // DELETE api/<OrderController>/5
        [HttpDelete("{id}")]
        public IActionResult Delete(string id)
        {
            _orderService.Delete(id);
            return new NoContentResult();
        }
    }
}
